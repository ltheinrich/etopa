//! Data API handlers

use crate::{common::*, SharedData};
use etopa::{
    data::{open_file, read_file, write_file},
    Fail,
};
use lhi::server::{respond, HttpRequest};
use std::sync::{Arc, RwLock};

/// Get encrypted secure storage file handler
pub fn get_secure(req: HttpRequest, shared: Arc<RwLock<SharedData>>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;

    // get shared
    let shared = shared.read().unwrap();

    // verify login
    if shared.user_logins.valid(username, token) {
        // read storage file
        let mut file = open_file(format!("{}/{}.edb", shared.data_dir, username))?;
        Ok(respond(
            read_file(&mut file)?,
            "application/octet-stream",
            cors_headers(),
        ))
    } else {
        // wrong login token
        Fail::from("unauthenticated")
    }
}

/// Set encrypted secure storage file handler
pub fn set_secure(req: HttpRequest, shared: Arc<RwLock<SharedData>>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;

    // get shared
    let shared = shared.read().unwrap();

    // verify login
    if shared.user_logins.valid(username, token) {
        // write storage file
        let mut file = open_file(format!("{}/{}.edb", shared.data_dir, username))?;
        write_file(&mut file, req.body())?;

        // return success
        Ok(jsonify(object!(success: true)))
    } else {
        // wrong login token
        Fail::from("unauthenticated")
    }
}
