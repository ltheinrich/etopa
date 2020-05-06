//! API utils

use etopa::crypto::random_an;
use etopa::Fail;
use json::JsonValue;
use lhi::server::{respond, ResponseData};
use std::collections::BTreeMap;
use std::fmt::Display;
use std::str::FromStr;
use std::time::{Duration, SystemTime};

/// Get value as string or fail
pub fn get_str<'a>(data: &BTreeMap<String, &'a str>, key: &str) -> Result<&'a str, Fail> {
    Ok(*data
        .get(key)
        .ok_or_else(|| Fail::new(format!("{} required", key)))?)
}

/// Get value or fail
pub fn get<T: FromStr>(data: &BTreeMap<String, &str>, key: &str) -> Result<T, Fail> {
    get_str(data, key)?
        .parse()
        .or_else(|_| Fail::from(format!("{} is not correct type", key)))
}

/// Get alphanumeric value as string or fail
pub fn get_an<'a>(data: &BTreeMap<String, &'a str>, key: &str) -> Result<&'a str, Fail> {
    // get string
    let an = get_str(data, key)?;

    // check if alphanumeric
    if !an.chars().all(char::is_alphanumeric) {
        return Fail::from(format!("{} is not alphanumeric", key));
    }

    // return string
    Ok(an)
}

/// Get username string and check if alphanumeric
pub fn get_username<'a>(data: &BTreeMap<String, &'a str>) -> Result<&'a str, Fail> {
    get_an(data, "username")
}

/// Convert JsonValue to response
pub fn jsonify(value: JsonValue) -> Vec<u8> {
    respond(value.to_string(), "application/json", cors_headers())
}

/// Convert error message into json format error
pub fn json_error<E: Display>(err: E) -> Vec<u8> {
    jsonify(object!(error: format!("{}", err)))
}

pub fn cors_headers() -> Option<ResponseData<'static>> {
    let mut resp_data = ResponseData::new();
    resp_data.headers.insert("access-control-allow-origin", "*");
    resp_data
        .headers
        .insert("access-control-allow-headers", "*");
    resp_data
        .headers
        .insert("access-control-allow-methods", "*");
    Some(resp_data)
}

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
