//! Etopa main

extern crate aes_gcm;
extern crate kern;
extern crate rand;
extern crate rustls;

mod common;
mod data;
mod handler;
mod http;

use common::*;
use handler::*;
use kern::cli::Command;
use kern::Error;
use rustls::internal::pemfile::{certs, pkcs8_private_keys, rsa_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use std::error::Error as StdError;
use std::fs::File;
use std::io::prelude::Read;
use std::io::BufReader;
use std::net::TcpListener;
use std::sync::{Arc, RwLock};
use std::{env, thread};

// Main function
fn main() {
    use aes_gcm::aead::{generic_array::GenericArray, Aead, NewAead};
    use aes_gcm::Aes256Gcm;
    let key = GenericArray::clone_from_slice(b"12345678901234567890123456789012");
    let aead = Aes256Gcm::new(key);
    let nonce_buf: Vec<u8> = (0..32).map(|_| rand::random()).collect();
    let nonce = GenericArray::from_slice(&nonce_buf);
    let ciphertext = aead
        .encrypt(nonce, b"plaintext message".as_ref())
        .expect("encryption failure!");
    let plaintext = aead
        .decrypt(nonce, ciphertext.as_ref())
        .expect("decryption failure!");
    println!("{}", String::from_utf8(plaintext).unwrap());

    // init
    init_version();

    // parse arguments
    let args: Vec<String> = env::args().collect();
    let cmd = Command::from(&args, &["help"]);
    if cmd.is_option("help") {
        return println!("{}", HELP);
    }

    // config options
    let mut port = "4490";
    let mut addr = "[::]";
    let mut threads = 1; // additional threads
    let mut cert = "cert.pem";
    let mut key = "key.pem";

    // parse config file
    let mut buf = String::new();
    if let Ok(mut file) = File::open("/etc/etopa.conf") {
        if file.read_to_string(&mut buf).is_ok() {
            conf_file(
                &mut buf,
                &mut port,
                &mut addr,
                &mut threads,
                &mut cert,
                &mut key,
            );
        }
    }

    // parse cli config
    conf_cli(
        &cmd,
        &mut port,
        &mut addr,
        &mut threads,
        &mut cert,
        &mut key,
    );

    // start server
    let tls_config = match gen_tls_config(cert, key) {
        Ok(tls_config) => tls_config,
        Err(err) => return println!("{}", err),
    };
    let listener = TcpListener::bind(format!("{}:{}", addr, port)).expect("Address already in use");
    let listener = Arc::new(RwLock::new(listener));

    // start threads
    (0..threads).for_each(|_| {
        let listener = listener.clone();
        let tls_config = tls_config.clone();
        thread::spawn(move || accept_connections(listener, tls_config));
    });

    // print info message
    println!("HTTPS server will be available on {}:{}", addr, port);

    // main thread
    thread::spawn(move || accept_connections(listener, tls_config))
        .join()
        .unwrap();
}

// Generate TLS configuration
fn gen_tls_config(cert_path: &str, key_path: &str) -> Result<Arc<ServerConfig>, Box<dyn StdError>> {
    let mut config = ServerConfig::new(NoClientAuth::new());
    let mut cert_buf = BufReader::new(File::open(cert_path)?);
    let cert = match certs(&mut cert_buf) {
        Ok(key) => key,
        Err(_) => return Err(Box::new(Error::new("broken certificate"))),
    };
    let mut key_buf = BufReader::new(File::open(key_path)?);
    let key = match rsa_private_keys(&mut key_buf) {
        Ok(key) => {
            if !key.is_empty() {
                key[0].clone()
            } else {
                let mut key_buf = BufReader::new(File::open(key_path)?);
                match pkcs8_private_keys(&mut key_buf) {
                    Ok(key) => {
                        if !key.is_empty() {
                            key[0].clone()
                        } else {
                            return Err(Box::new(Error::new("broken private key")));
                        }
                    }
                    Err(_) => return Err(Box::new(Error::new("broken private key"))),
                }
            }
        }
        Err(_) => {
            let mut key_buf = BufReader::new(File::open(key_path)?);
            match pkcs8_private_keys(&mut key_buf) {
                Ok(key) => {
                    if !key.is_empty() {
                        key[0].clone()
                    } else {
                        return Err(Box::new(Error::new("broken private key")));
                    }
                }
                Err(_) => return Err(Box::new(Error::new("broken private key"))),
            }
        }
    };
    config.set_single_cert(cert, key)?;
    Ok(Arc::new(config))
}

// Get cli configuration
fn conf_cli<'a>(
    cmd: &'a Command<'_>,
    port: &mut &'a str,
    addr: &mut &'a str,
    threads: &mut u8,
    cert: &mut &'a str,
    key: &mut &'a str,
) {
    // parse command-line options
    if let Some(v) = cmd.get_parameter("port") {
        *port = v;
    }
    if let Some(v) = cmd.get_parameter("addr") {
        *addr = v;
    }
    if let Some(v) = cmd.get_parameter("threads") {
        // parse to u8
        if let Ok(v) = v.parse() {
            *threads = v;
        }
    }
    if let Some(v) = cmd.get_parameter("cert") {
        *cert = v;
    }
    if let Some(v) = cmd.get_parameter("key") {
        *key = v;
    }
}

// Parse file config
fn conf_file<'a>(
    buf: &'a mut String,
    port: &mut &'a str,
    addr: &mut &'a str,
    threads: &mut u8,
    cert: &mut &'a str,
    key: &mut &'a str,
) {
    // parse file config
    buf.split('\n') // split lines
        .map(|l| l.splitn(2, '=').map(|c| c.trim()).collect()) // seperate and trim key and value
        .for_each(|kv: Vec<&str>| {
            if kv.len() == 2 {
                match kv[0] {
                    "port" => *port = kv[1],
                    "addr" => *addr = kv[1],
                    "threads" => {
                        // parse to u8
                        if let Ok(v) = kv[1].parse() {
                            *threads = v;
                        }
                    }
                    "cert" => *cert = kv[1],
                    "key" => *key = kv[1],
                    _ => {}
                }
            }
        });
}
