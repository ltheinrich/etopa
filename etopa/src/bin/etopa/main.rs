//! Etopa main

use etopa::common::*;
use kern::{init_version, Command, Config, Fail};
use lhi::server::{listen, load_certificate, HttpSettings};
use std::env::args;

/// Main function
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
    println!(
        "Etopa {} (c) 2020 Lennart Heinrich\n",
        init_version(CARGO_TOML)
    );

    // parse arguments
    let args: Vec<String> = args().collect();
    let cmd = Command::from(&args, &["help"]);
    if cmd.option("help") {
        return println!("{}", HELP);
    }

    // load file config
    let mut conf_buf = String::new();
    let config =
        Config::read("/etc/etopa.conf", &mut conf_buf).unwrap_or_else(|_| Config::from(""));

    // configuration
    let port = cmd.param("port", config.value("port", "4490"));
    let addr = cmd.param("addr", config.value("addr", "[::]"));
    let threads = cmd.parameter("threads", config.get("threads", 1));
    let cert = cmd.param("cert", config.value("cert", "cert.pem"));
    let key = cmd.param("key", config.value("key", "key.pem"));

    // start server
    let tls_config = load_certificate(cert, key).unwrap();
    let listeners = listen(
        &format!("{}:{}", addr, port),
        threads,
        HttpSettings::new(),
        tls_config,
        |_| Fail::from("unimplemented"),
    )
    .unwrap();

    // print info message and join threads
    println!("HTTPS server available on {}:{}", addr, port);
    for listener in listeners {
        listener.join().expect("listener thread crashed");
    }
}
