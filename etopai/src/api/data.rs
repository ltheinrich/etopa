//! Data API handlers

use crate::data::{open_file, read_file, StorageFile};
use crate::{common::*, SharedData};
use etopa::Fail;
use lhi::server::{respond, HttpRequest};
use std::sync::RwLockReadGuard;

/// Get storage file handler
pub fn get_secure(
    req: HttpRequest,
    shared: RwLockReadGuard<'_, SharedData>,
) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;

    // verify login
    if shared.logins().valid(username, token) {
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

/// Update storage file handler
pub fn update(req: HttpRequest, shared: RwLockReadGuard<'_, SharedData>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;
    let secret_name = get_str(headers, "secret_name")?;
    let secret_value = get_str(headers, "secret_value")?;
    let secret_name_encrypted = get_str(headers, "secret_name_encrypted")?;

    // verify login
    if shared.logins().valid(username, token) {
        // read storage file
        let mut storage = StorageFile::new(format!("{}/{}.edb", shared.data_dir, username))?;
        let cache = storage.cache_mut();

        // update in storage file
        cache.insert(format!("{}_secret", secret_name), secret_value.to_string());
        cache.insert(
            format!("{}_secret_name", secret_name),
            secret_name_encrypted.to_string(),
        );
        storage.write()?;

        // return success
        Ok(jsonify(object!(error: false)))
    } else {
        // wrong login token
        Fail::from("unauthenticated")
    }
}

/// Delete from storage file handler
pub fn delete(req: HttpRequest, shared: RwLockReadGuard<'_, SharedData>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;
    let secret_name = get_str(headers, "secret_name")?;

    // verify login
    if shared.logins().valid(username, token) {
        // read storage file
        let mut storage = StorageFile::new(format!("{}/{}.edb", shared.data_dir, username))?;
        let cache = storage.cache_mut();

        // update in storage file
        cache.remove(&format!("{}_secret", secret_name));
        cache.remove(&format!("{}_secret_name", secret_name));
        storage.write()?;

        // return success
        Ok(jsonify(object!(error: false)))
    } else {
        // wrong login token
        Fail::from("unauthenticated")
    }
}
