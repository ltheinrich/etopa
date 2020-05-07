//! User API handlers

use crate::data::move_file;
use crate::utils::*;
use crate::{jsonify, SharedData};
use etopa::crypto::argon2_verify;
use etopa::Fail;
use lhi::server::HttpRequest;
use std::fs::remove_file;
use std::sync::RwLockReadGuard;

/// Token validation handler
pub fn valid(req: HttpRequest, shared: RwLockReadGuard<'_, SharedData>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;

    // validate
    Ok(jsonify(
        object!(valid: shared.logins().valid(username, token)),
    ))
}

/// Account logout handler
pub fn logout(req: HttpRequest, shared: RwLockReadGuard<'_, SharedData>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;

    // verify login
    if shared.logins().valid(username, token) {
        // delete user token
        shared.logins_mut().remove(username, token);

        // successfully deleted
        Ok(jsonify(object!(error: false)))
    } else {
        // wrong login token
        Fail::from("unauthenticated")
    }
}

/// Account deletion handler
pub fn delete(req: HttpRequest, shared: RwLockReadGuard<'_, SharedData>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;

    // verify login
    if shared.logins().valid(username, token) {
        // delete user
        let mut users = shared.users_mut();
        users.cache_mut().remove(username);
        users.write()?;
        shared.logins_mut().remove_user(username);
        remove_file(format!("{}/{}.edb", shared.data_dir, username)).ok();

        // successfully deleted
        Ok(jsonify(object!(error: false)))
    } else {
        // wrong login token
        Fail::from("unauthenticated")
    }
}

/// Login handler
pub fn login(req: HttpRequest, shared: RwLockReadGuard<'_, SharedData>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let password = get_str(headers, "password")?;

    // get password hash from db
    match shared.users().cache().get(username) {
        Some(password_hash) => {
            // verify argon2 password hash
            let password_verified = argon2_verify(password_hash, password.as_bytes());
            if !password_verified {
                return Fail::from("unauthenticated");
            }

            // return login token
            Ok(jsonify(object!(token: shared.logins_mut().add(username))))
        }
        None => Fail::from("unauthenticated"),
    }
}

/// Account registration handler
pub fn register(
    req: HttpRequest,
    shared: RwLockReadGuard<'_, SharedData>,
) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let password = get_str(headers, "password")?;

    // check if user already exists
    if shared.users().cache().contains_key(username) {
        return Fail::from("username already exists");
    }

    // modify users
    let mut users = shared.users_mut();
    users
        .cache_mut()
        .insert(username.to_string(), password.to_string());
    users.write()?;

    // return login token
    Ok(jsonify(object!(token: shared.logins_mut().add(username))))
}

/// Change username handler
pub fn change_username(
    req: HttpRequest,
    shared: RwLockReadGuard<'_, SharedData>,
) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;
    let new_username = get_an(headers, "newusername")?;

    // verify login
    if shared.logins().valid(username, token) {
        // update secure storage
        let edb_path = format!("{}/{}.edb", shared.data_dir, username);

        // check if user already exists
        if shared.users().cache().contains_key(new_username) {
            return Fail::from("new username already exists");
        }

        // move storage file
        let new_edb_path = format!("{}/{}.edb", shared.data_dir, new_username);
        move_file(&edb_path, &new_edb_path)?;

        // change username
        let mut users = shared.users_mut();
        match users.cache_mut().remove(username) {
            Some(password_hash) => users
                .cache_mut()
                .insert(new_username.to_string(), password_hash),
            None => {
                // revert filename change
                move_file(&new_edb_path, &edb_path)?;
                return Fail::from("internal error: user entry does not exist in cache");
            }
        };

        // change users file
        if let Err(err) = users.write() {
            // revert filename change
            move_file(&new_edb_path, &edb_path)?;
            return Err(err);
        }

        // rename user login tokens and return success
        shared
            .logins_mut()
            .rename(username, new_username.to_string());
        Ok(jsonify(object!(error: false)))
    } else {
        Fail::from("unauthenticated")
    }
}

/// Change password handler
pub fn change_password(
    req: HttpRequest,
    shared: RwLockReadGuard<'_, SharedData>,
) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;
    let new_password = get_str(headers, "newpassword")?;

    // verify login
    if shared.logins().valid(username, token) {
        // change password
        let mut users = shared.users_mut();
        match users.cache_mut().get_mut(username) {
            Some(password_hash) => *password_hash = new_password.to_string(),
            None => {
                return Fail::from("internal error: user entry does not exist in cache");
            }
        };

        // write changes and return success
        users.write()?;
        Ok(jsonify(object!(error: false)))
    } else {
        Fail::from("unauthenticated")
    }
}
