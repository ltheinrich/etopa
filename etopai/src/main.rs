//! Etopa HTTPS API

#[macro_use]
extern crate json;

mod api;

use etopa::common::*;
use etopa::{init_version, Command, Config, Fail};
use json::JsonValue;
use lhi::server::{listen, load_certificate, respond, HttpRequest, HttpSettings};
use std::env::args;
use std::fmt::Display;

/// Main function
fn main() {
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
    let cert = cmd.param("cert", config.value("cert", "data/cert.pem"));
    let key = cmd.param("key", config.value("key", "data/key.pem"));

    // start server
    let tls_config = load_certificate(cert, key).unwrap();
    let listeners = listen(
        &format!("{}:{}", addr, port),
        threads,
        HttpSettings::new(),
        tls_config,
        handle,
    )
    .unwrap();

    // print info message and join threads
    println!("HTTPS server available on {}:{}", addr, port);
    for listener in listeners {
        listener.join().expect("listener thread crashed");
    }
}

/// Assigning requests to handlers
fn handle(req: Result<HttpRequest, Fail>) -> Result<Vec<u8>, Fail> {
    // unwrap and match url
    let req: HttpRequest = req?;
    let handler = match req.url() {
        "/test" => api::test,
        // handler not found
        _ => return Ok(json_error("Handler not found")),
    };

    // handle request
    Ok(handler(req))
}

/// Convert JsonValue to response
pub fn jsonify(value: JsonValue) -> Vec<u8> {
    respond(value.to_string().as_bytes(), "application/json", None)
}

/// Convert error message into json format error
pub fn json_error<E: Display>(err: E) -> Vec<u8> {
    jsonify(object!(error: format!("{}", err)))
}
