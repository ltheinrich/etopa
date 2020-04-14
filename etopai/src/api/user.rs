//! User API handlers

use crate::utils::*;
use crate::{jsonify, SharedData};
use etopa::random_an;
use etopa::{argon2_hash, argon2_verify, random, Fail};
use lhi::server::HttpRequest;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};

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

/// Account deletion handler
pub fn delete(req: HttpRequest, shared: Arc<RwLock<SharedData>>) -> Result<Vec<u8>, Fail> {
    // parse
    let val = default_check(&req)?;
    let data = to_map(&val);

    // get values
    let username = get_str(&data, "username")?;
    let token = get_str(&data, "token")?;

    // get shared
    let mut shared = shared.write().unwrap();
    let mut users = shared.user_data.parse()?;

    // verify login
    if shared.user_logins.valid(username, token) {
        // delete user
        users.remove(username);
        shared.user_data.serialize(&users)?;
        shared.user_logins.remove_user(username);

        // successfully deleted
        Ok(jsonify(object!(success: true)))
    } else {
        // wrong login token
        Fail::from("unauthenticated")
    }
}

/// Login handler
pub fn login(req: HttpRequest, shared: Arc<RwLock<SharedData>>) -> Result<Vec<u8>, Fail> {
    // parse
    let val = default_check(&req)?;
    let data = to_map(&val);

    // get values
    let username = get_str(&data, "username")?;
    let password = get_str(&data, "password")?;

    // get shared
    let mut shared = shared.write().unwrap();
    let users = shared.user_data.parse()?;

    // get password hash from db
    match users.get(username) {
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

/// Account creation handler
pub fn create(req: HttpRequest, shared: Arc<RwLock<SharedData>>) -> Result<Vec<u8>, Fail> {
    // parse
    let val = default_check(&req)?;
    let data = to_map(&val);

    // get values
    let username = get_str(&data, "username")?;
    let password = get_str(&data, "password")?;

    // get shared
    let mut shared = shared.write().unwrap();
    let mut users = shared.user_data.parse()?;

    // check if user already exists
    if users.contains_key(username) {
        return Fail::from("username already exists");
    }

    // argon2 hash password
    let salt = random(10);
    let password_argon2 = argon2_hash(password.as_bytes(), &salt)?;

    // modify users
    users.insert(username.to_string(), password_argon2);
    shared.user_data.serialize(&users)?;

    // return login token
    Ok(jsonify(object!(token: shared.user_logins.add(username))))
}
