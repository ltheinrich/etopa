//! Etopa main

mod config;

use config::{conf_cli, conf_file};
use etopa::common::*;
use kern::cli::Command;
use lhi::{
    server::{listen, load_certificate},
    HttpError,
};
use std::env::args;
use std::fs::File;
use std::io::prelude::Read;

// Main function
fn main() {
    use aes_gcm::aead::{generic_array::GenericArray, Aead, NewAead};
    use aes_gcm::Aes256Gcm;
    let key = GenericArray::clone_from_slice(b"12345678901234567890123456789012");
    let aead = Aes256Gcm::new(key);
    let nonce_buf: Vec<u8> = (0..12).map(|_| rand::random()).collect();
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
    let args: Vec<String> = args().collect();
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
    let tls_config = load_certificate(cert, key).unwrap();
    let listeners = listen(&format!("{}:{}", addr, port), threads, tls_config, |_| {
        HttpError::from("unimplemented")
    })
    .unwrap();

    // print info message and join threads
    println!("HTTPS server available on {}:{}", addr, port);
    for listener in listeners {
        listener.join().expect("listener thread crashed");
    }
}
