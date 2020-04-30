//! Etopa HTTPS API

#[macro_use]
extern crate json;

pub mod common;
pub mod utils;

mod api;

use common::{json_error, jsonify, SharedData, CARGO_TOML, HELP};
use etopa::{data::StorageFile, meta::init_version, Command, Config, Fail};
use lhi::server::{listen, load_certificate, HttpRequest, HttpSettings};
use std::env::args;
use std::sync::{Arc, RwLock};

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
    let data = cmd.param("data", config.value("data", "data"));
    let cert = cmd.parameter("cert", config.get("cert", format!("{}/cert.pem", data)));
    let key = cmd.parameter("key", config.get("key", format!("{}/key.pem", data)));

    // open username/password storage
    let user_data = StorageFile::new(&format!("{}/user_data.esdb", data)).unwrap();

    // start server
    let tls_config = load_certificate(&cert, &key).unwrap();
    let listeners = listen(
        &format!("{}:{}", addr, port),
        threads,
        HttpSettings::new(),
        tls_config,
        handle,
        Arc::new(RwLock::new(SharedData::new(user_data, data.to_string()))),
    )
    .unwrap();

    // print info message and join threads
    println!("HTTPS server available on {}:{}", addr, port);
    for listener in listeners {
        listener.join().expect("listener thread crashed");
    }
}

/// Assigning requests to handlers
fn handle(
    req: Result<HttpRequest, Fail>,
    shared: Arc<RwLock<SharedData>>,
) -> Result<Vec<u8>, Fail> {
    // unwrap and match url
    let req: HttpRequest = req?;
    let handler = match req.url() {
        "/user/register" => api::user::register,
        "/user/login" => api::user::login,
        "/user/delete" => api::user::delete,
        "/user/logout" => api::user::logout,
        "/user/valid" => api::user::valid,
        "/user/update" => api::user::update,
        "/data/get_secure" => api::data::get_secure,
        "/data/update" => api::data::update,
        "/data/delete" => api::data::delete,
        // handler not found
        _ => return Ok(json_error("handler not found")),
    };

    // handle request
    Ok(match handler(req, shared) {
        Ok(resp) => resp,
        Err(err) => json_error(err),
    })
}
