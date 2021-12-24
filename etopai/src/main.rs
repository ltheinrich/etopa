//! Etopa HTTPS API

#[macro_use]
extern crate json;

mod common;
mod utils;

mod api;

use common::{json_error, SharedData, BUILD_GRADLE, HELP, LICENSES, TLS_CERTIFICATE, TLS_KEY};
use etopa::{meta::search, CliBuilder, Config, Fail};
use kern::data::StorageFile;
use kern::http::server::{certificate_config, listen, load_certificate, HttpRequest, HttpSettings};
use std::env::args;
use std::fs::create_dir_all;
use std::sync::{Arc, RwLock};
use utils::{ApiAction, SecurityManager};

/// Main function
fn main() {
    // init
    println!(
        "Etopa {} (c) 2020 Lennart Heinrich\n",
        search(BUILD_GRADLE, "versionName").unwrap_or("0.0.0")
    );

    // parse arguments
    let args: Vec<String> = args().collect();
    let cmd = CliBuilder::new()
        .options(&["help", "licenses", "nolog"])
        .build(&args);
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
    let vlt: u64 = cmd.parameter("vlt", config.get("vlt", 604_800));
    let ban_time: u64 = cmd.parameter("bantime", config.get("bantime", 3600));
    let login_fails: u32 = cmd.parameter("loginfails", config.get("loginfails", 50));
    let login_time: u64 = cmd.parameter("logintime", config.get("logintime", 60));
    let account_limit: u32 = cmd.parameter("acclimit", config.get("acclimit", 10));
    let data = cmd.param("data", config.value("data", "data"));
    let cert = cmd.parameter("cert", config.get("cert", format!("{}/cert.pem", data)));
    let key = cmd.parameter("key", config.get("key", format!("{}/key.pem", data)));
    let log = !cmd.option("nolog") && !config.get("nolog", false);

    // open username/password storage
    create_dir_all(data).ok();
    let users = StorageFile::new(&format!("{}/users.esdb", data)).unwrap();

    // start server
    let security = SecurityManager::new(ban_time, login_fails, login_time, account_limit, log);
    let tls_config = match load_certificate(&cert, &key) {
        Ok(config) => config,
        Err(err) => {
            eprintln!(
                "Could not load TLS certificate or key:\n{}\nWARNING! Default certificate and key\n",
                err
            );
            certificate_config(TLS_CERTIFICATE, TLS_KEY).unwrap()
        }
    };
    let listeners = listen(
        &format!("{}:{}", addr, port),
        threads,
        HttpSettings::new(),
        tls_config,
        handle,
        Arc::new(RwLock::new(SharedData::new(
            users,
            security,
            vlt,
            data.to_string(),
            log,
        ))),
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
        "/data/update_sort" => api::data::update_sort,
        // handler not found
        _ => return Ok(json_error("handler not found")),
    };

    // check ip address
    if !shared
        .read()
        .unwrap()
        .security()
        .check(req.ip(), ApiAction::Simple)
    {
        return Ok(json_error("blocked ip address"));
    }

    // handle request
    Ok(match handler(req, shared.read().unwrap()) {
        Ok(resp) => resp,
        Err(err) => json_error(err),
    })
}
