//! Etopa HTTPS API

#[macro_use]
extern crate json;

mod common;
mod data;
mod utils;

mod api;

use common::{json_error, SharedData, BUILD_GRADLE, HELP, LICENSES};
use data::StorageFile;
use etopa::{meta::search, CliBuilder, Config, Fail};
use kern::http::server::{listen, load_certificate, HttpRequest, HttpSettings};
use std::env::args;
use std::fs::create_dir_all;
use std::sync::{Arc, RwLock};

/// Main function
fn main() {
    // init
    println!(
        "Etopa {} (c) 2020 Lennart Heinrich\n",
        search(BUILD_GRADLE, "versionName").unwrap_or("0.0.0")
    );

    // parse arguments
    let args: Vec<String> = args().collect();
    let cmd = CliBuilder::new().options(&["help"]).build(&args);
    if cmd.option("help") {
        return println!("{}", HELP);
    } else if cmd.option("licenses") {
        return println!("{}", LICENSES);
    }

    // load file config
    let mut conf_buf = String::new();
    let config =
        Config::read("/etc/etopa.conf", &mut conf_buf).unwrap_or_else(|_| Config::from(""));

    // configuration
    let port = cmd.param("port", config.value("port", "4490"));
    let addr = cmd.param("addr", config.value("addr", "[::]"));
    let threads = cmd.parameter("threads", config.get("threads", 2));
    let vlt = cmd.parameter("vlt", config.get("vlt", 604_800));
    let data = cmd.param("data", config.value("data", "data"));
    let cert = cmd.parameter("cert", config.get("cert", format!("{}/cert.pem", data)));
    let key = cmd.parameter("key", config.get("key", format!("{}/key.pem", data)));

    // open username/password storage
    create_dir_all(data).ok();
    let users = StorageFile::new(&format!("{}/users.esdb", data)).unwrap();

    // start server
    let tls_config = load_certificate(&cert, &key).unwrap();
    let listeners = listen(
        &format!("{}:{}", addr, port),
        threads,
        HttpSettings::new(),
        tls_config,
        handle,
        Arc::new(RwLock::new(SharedData::new(users, vlt, data.to_string()))),
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
        // user
        "/user/register" => api::user::register,
        "/user/login" => api::user::login,
        "/user/delete" => api::user::delete,
        "/user/logout" => api::user::logout,
        "/user/valid" => api::user::valid,
        "/user/change_password" => api::user::change_password,
        "/user/change_username" => api::user::change_username,
        // data
        "/data/get_secure" => api::data::get_secure,
        "/data/set_secure" => api::data::set_secure,
        "/data/update" => api::data::update,
        "/data/delete" => api::data::delete,
        "/data/rename" => api::data::rename,
        // handler not found
        _ => return Ok(json_error("handler not found")),
    };

    // handle request
    Ok(match handler(req, shared.read().unwrap()) {
        Ok(resp) => resp,
        Err(err) => json_error(err),
    })
}
