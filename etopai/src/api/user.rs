//! User API handlers

use crate::utils::*;
use crate::{jsonify, SharedData};
use etopa::Fail;
use etopa::{
    crypto::{argon2_hash, argon2_verify, random, random_an},
    data::{move_file, open_file, write_file},
};
use lhi::server::HttpRequest;
use std::collections::BTreeMap;
use std::fs::remove_file;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};

/// Seconds a login token is valid
const VALID_LOGIN_SECS: u64 = 3600;

/// User login/token management
#[derive(Clone, Debug, Default)]
pub struct UserLogins {
    user_logins: BTreeMap<String, Vec<(String, SystemTime)>>,
}

impl UserLogins {
    /// Create empty
    pub fn new() -> Self {
        Self {
            user_logins: BTreeMap::new(),
        }
    }

    /// Check if login token is valid and remove expired
    pub fn valid(&self, user: &str, token: &str) -> bool {
        // get logins
        match self.user_logins.get(user) {
            Some(logins) => {
                // check login
                logins
                    .iter()
                    .any(|login| login.0 == token && Self::check_unexpired(&login.1))
            }
            None => false,
        }
    }

    /// Generate login token for user
    pub fn add(&mut self, user: &str) -> &str {
        // generate random token and get logins
        let token = random_an(32);
        match self.user_logins.get_mut(user) {
            Some(logins) => {
                // remove expired logins and return logins
                Self::remove_expired(logins);
                logins.push((token, SystemTime::now()));
            }
            None => {
                // create new logins vector for user
                self.user_logins
                    .insert(user.to_string(), [(token, SystemTime::now())].to_vec());
            }
        };

        // return token
        &self.user_logins[user].last().unwrap().0
    }

    /// Remove login token for user
    pub fn remove(&mut self, user: &str, token: &str) {
        // get logins
        if let Some(logins) = self.user_logins.get_mut(user) {
            // remove token
            logins.retain(|login| login.0 != token && Self::check_unexpired(&login.1));
        }
    }

    /// Remove all logins for user
    pub fn remove_user(&mut self, user: &str) {
        // remove user
        self.user_logins.remove(user);
    }

    /// Rename user entry
    pub fn rename(&mut self, user: &str, new_user: String) {
        if let Some(logins) = self.user_logins.remove(user) {
            self.user_logins.insert(new_user, logins);
        }
    }

    /// Remove expired logins
    fn remove_expired(logins: &mut Vec<(String, SystemTime)>) {
        (*logins).retain(|login| Self::check_unexpired(&login.1));
    }

    /// Check if login is expired
    fn check_unexpired(expiration: &SystemTime) -> bool {
        expiration
            .elapsed()
            .unwrap_or(Duration::from_secs(u64::max_value()))
            .as_secs()
            < VALID_LOGIN_SECS
    }
}

/// Token validation handler
pub fn valid(req: HttpRequest, shared: Arc<RwLock<SharedData>>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;

    // get shared and validate
    let shared = shared.read().unwrap();
    Ok(jsonify(
        object!(valid: shared.user_logins.valid(username, token)),
    ))
}

/// Account logout handler
pub fn logout(req: HttpRequest, shared: Arc<RwLock<SharedData>>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;

    // get shared
    let mut shared = shared.write().unwrap();

    // verify login
    if shared.user_logins.valid(username, token) {
        // delete user token
        shared.user_logins.remove(username, token);

        // successfully deleted
        Ok(jsonify(object!(error: false)))
    } else {
        // wrong login token
        Fail::from("unauthenticated")
    }
}

/// Account deletion handler
pub fn delete(req: HttpRequest, shared: Arc<RwLock<SharedData>>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;

    // get shared
    let mut shared = shared.write().unwrap();

    // verify login
    if shared.user_logins.valid(username, token) {
        // delete user
        shared.user_data.cache_mut().remove(username);
        shared.user_data.write()?;
        shared.user_logins.remove_user(username);
        remove_file(format!("{}/{}.edb", shared.data_dir, username)).ok();

        // successfully deleted
        Ok(jsonify(object!(error: false)))
    } else {
        // wrong login token
        Fail::from("unauthenticated")
    }
}

/// Login handler
pub fn login(req: HttpRequest, shared: Arc<RwLock<SharedData>>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let password = get_str(headers, "password")?;

    // get shared
    let mut shared = shared.write().unwrap();

    // get password hash from db
    match shared.user_data.cache().get(username) {
        Some(password_hash) => {
            // verify argon2 password hash
            let password_verified = argon2_verify(password_hash, password.as_bytes());
            if !password_verified {
                return Fail::from("unauthenticated");
            }

            // return login token
            Ok(jsonify(object!(token: shared.user_logins.add(username))))
        }
        None => Fail::from("unauthenticated"),
    }
}

/// Account registration handler
pub fn register(req: HttpRequest, shared: Arc<RwLock<SharedData>>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let password = get_str(headers, "password")?;

    // get shared
    let mut shared = shared.write().unwrap();
    let users = shared.user_data.cache_mut();

    // check if user already exists
    if users.contains_key(username) {
        return Fail::from("username already exists");
    }

    // argon2 hash password
    let salt = random(10);
    let password_argon2 = argon2_hash(password.as_bytes(), &salt)?;

    // modify users
    users.insert(username.to_string(), password_argon2);
    shared.user_data.write()?;

    // return login token
    Ok(jsonify(object!(token: shared.user_logins.add(username))))
}

/// Update user handler
pub fn update(req: HttpRequest, shared: Arc<RwLock<SharedData>>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;
    let password = get_str(headers, "password")?;
    let new_username = get_an(headers, "new_username");

    // get shared
    let mut shared = shared.write().unwrap();

    // verify login
    if shared.user_logins.valid(username, token) {
        // update secure storage
        let edb_path = format!("{}/{}.edb", shared.data_dir, username);
        let mut file = open_file(&edb_path)?;
        write_file(&mut file, req.body())?;

        // change password
        if let Some(user_password) = shared.user_data.cache_mut().get_mut(username) {
            // argon2 hash password
            let salt = random(10);
            let password_argon2 = argon2_hash(password.as_bytes(), &salt)?;

            // change password
            *user_password = password_argon2;
            shared.user_data.write()?;

            // remove user login tokens
            shared.user_logins.remove_user(username);
        } else {
            return Fail::from("internal error: user entry does not exist in cache");
        }

        // change username
        if let Ok(new_username) = new_username {
            // check if user already exists
            if shared.user_data.cache().contains_key(new_username) {
                return Fail::from("new username already exists");
            }

            // move storage file
            let new_edb_path = format!("{}/{}.edb", shared.data_dir, new_username);
            move_file(&edb_path, &new_edb_path)?;

            // change username
            let users = shared.user_data.cache_mut();
            match users.remove(username) {
                Some(password_hash) => users.insert(new_username.to_string(), password_hash),
                None => {
                    // revert filename change
                    move_file(&new_edb_path, &edb_path)?;
                    return Fail::from("internal error: user entry does not exist in cache");
                }
            };

            // change users file
            if let Err(err) = shared.user_data.write() {
                // revert filename change
                move_file(&new_edb_path, &edb_path)?;
                return Err(err);
            }

            // remove user login tokens
            shared.user_logins.remove_user(username);
        }

        // return success
        Ok(jsonify(object!(error: false)))
    } else {
        Fail::from("unauthenticated")
    }
}
