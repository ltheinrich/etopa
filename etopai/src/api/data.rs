//! Data API handlers

use crate::{common::*, SharedData};
use etopa::{data::StorageFile, Fail};
use lhi::server::{respond, HttpRequest};
use std::sync::{Arc, RwLock};

/// Get encrypted secure storage file handler
pub fn get_secure(req: HttpRequest, shared: Arc<RwLock<SharedData>>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_str(headers, "username")?;
    let token = get_str(headers, "token")?;

    // get shared
    let shared = shared.read().unwrap();

    // verify login
    if shared.user_logins.valid(username, token) {
        // read storage file
        let mut file = StorageFile::new(format!("{}/{}.edb", shared.data_dir, username))?;
        Ok(respond(
            file.read()?,
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
    let username = get_str(headers, "username")?;
    let token = get_str(headers, "token")?;

    // get shared
    let shared = shared.read().unwrap();

    // verify login
    if shared.user_logins.valid(username, token) {
        // write storage file
        let mut file = StorageFile::new(format!("{}/{}.edb", shared.data_dir, username))?;
        file.write(req.body())?;

        // return success
        Ok(jsonify(object!(success: true)))
    } else {
        // wrong login token
        Fail::from("unauthenticated")
    }
}
