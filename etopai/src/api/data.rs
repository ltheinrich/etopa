//! Data API handlers

use crate::common::*;
use etopa::Fail;
use kern::http::server::{respond, HttpRequest};
use std::sync::RwLockReadGuard;

/// Get storage file handler
pub fn get_secure(req: HttpRequest, shared: RwLockReadGuard<SharedData>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;

    // verify login
    if shared.logins().valid(username, token) {
        // create storage file if not exists
        if !shared.files().exists(username) {
            shared.files_mut().create(username)?;
        }

        // get storage file
        let files = shared.files();
        let file = files.read(username);
        if let Ok(storage) = file {
            // return storage file
            Ok(respond(
                storage.raw(),
                "application/octet-stream",
                cors_headers(),
            ))
        } else {
            // empty storage file
            Ok(respond("", "application/octet-stream", cors_headers()))
        }
    } else {
        // wrong login token
        shared.security_mut().login_fail(req.ip());
        Fail::from("unauthenticated")
    }
}

/// Set storage file handler
pub fn set_secure(req: HttpRequest, shared: RwLockReadGuard<SharedData>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;

    // verify login
    if shared.logins().valid(username, token) {
        // create storage file if not exists
        if !shared.files().exists(username) {
            shared.files_mut().create(username)?;
        }

        // write to storage file
        let files = shared.files();
        let mut storage = files.write(username)?;
        let raw = String::from_utf8_lossy(req.body());
        storage.raw_write(raw.to_string())?;

        // return success
        Ok(jsonify(object!(error: false)))
    } else {
        // wrong login token
        shared.security_mut().login_fail(req.ip());
        Fail::from("unauthenticated")
    }
}

/// Update storage file secrets_sort handler
pub fn update_sort(req: HttpRequest, shared: RwLockReadGuard<SharedData>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;
    let secrets_sort = get_str(headers, "secretssort")?;

    // verify login
    if shared.logins().valid(username, token) {
        // create storage file if not exists
        if !shared.files().exists(username) {
            shared.files_mut().create(username)?;
        }

        // get storage file
        let files = shared.files();
        let mut storage = files.write(username)?;
        let cache = storage.cache_mut();

        // update in storage file
        cache.insert("secrets_sort".to_owned(), secrets_sort.to_owned());
        storage.write()?;

        // return success
        Ok(jsonify(object!(error: false)))
    } else {
        // wrong login token
        shared.security_mut().login_fail(req.ip());
        Fail::from("unauthenticated")
    }
}

/// Update storage file handler
pub fn update(req: HttpRequest, shared: RwLockReadGuard<SharedData>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;
    let secret_name = get_str(headers, "secretname")?;
    let secret_value = get_str(headers, "secretvalue")?;
    let secret_name_encrypted = get_str(headers, "secretnameencrypted")?;

    // verify login
    if shared.logins().valid(username, token) {
        // create storage file if not exists
        if !shared.files().exists(username) {
            shared.files_mut().create(username)?;
        }

        // get storage file
        let files = shared.files();
        let mut storage = files.write(username)?;
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
        shared.security_mut().login_fail(req.ip());
        Fail::from("unauthenticated")
    }
}

/// Rename storage file entry handler
pub fn rename(req: HttpRequest, shared: RwLockReadGuard<SharedData>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;
    let secret_name = get_str(headers, "secretname")?;
    let new_secret_name = get_str(headers, "newsecretname")?;
    let secret_name_encrypted = get_str(headers, "secretnameencrypted")?;

    // verify login
    if shared.logins().valid(username, token) {
        // create storage file if not exists
        if !shared.files().exists(username) {
            shared.files_mut().create(username)?;
        }

        // get storage file
        let files = shared.files();
        let mut storage = files.write(username)?;
        let cache = storage.cache_mut();

        // update in storage file
        let secret = cache
            .remove(&format!("{}_secret", secret_name))
            .ok_or_else(|| Fail::new(""))?;
        cache
            .remove(&format!("{}_secret_name", secret_name))
            .ok_or_else(|| Fail::new(""))?;
        cache.insert(format!("{}_secret", new_secret_name), secret);
        cache.insert(
            format!("{}_secret_name", new_secret_name),
            secret_name_encrypted.to_string(),
        );
        storage.write()?;

        // return success
        Ok(jsonify(object!(error: false)))
    } else {
        // wrong login token
        shared.security_mut().login_fail(req.ip());
        Fail::from("unauthenticated")
    }
}

/// Delete from storage file handler
pub fn delete(req: HttpRequest, shared: RwLockReadGuard<SharedData>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;
    let secret_name = get_str(headers, "secretname")?;

    // verify login
    if shared.logins().valid(username, token) {
        // create storage file if not exists
        if !shared.files().exists(username) {
            shared.files_mut().create(username)?;
        }

        // get storage file
        let files = shared.files();
        let mut storage = files.write(username)?;
        let cache = storage.cache_mut();

        // update in storage file
        cache.remove(&format!("{}_secret", secret_name));
        cache.remove(&format!("{}_secret_name", secret_name));
        storage.write()?;

        // return success
        Ok(jsonify(object!(error: false)))
    } else {
        // wrong login token
        shared.security_mut().login_fail(req.ip());
        Fail::from("unauthenticated")
    }
}
