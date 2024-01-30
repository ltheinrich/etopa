//! API utils

use etopa::crypto::random_an;
use etopa::{Fail, Result};
use json::JsonValue;
use kern::data::delete_file;
use kern::data::StorageFile;
use kern::http::server::{respond, ResponseData};
use kern::string::is_alphanumeric;
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::time::{Duration, SystemTime};

/// Get value as string or fail
pub fn get_str<'a>(data: &HashMap<String, &'a str>, key: &str) -> Result<&'a str> {
    Ok(*data
        .get(key)
        .ok_or_else(|| Fail::new(format!("{} required", key)))?)
}

/*
/// Get value or fail
pub fn get<T: FromStr>(data: &HashMap<String, &str>, key: &str) -> Result<T> {
    get_str(data, key)?
        .parse()
        .or_else(|_| Fail::from(format!("{} is not correct type", key)))
}
*/

/// Get alphanumeric value as string or fail
pub fn get_an<'a>(data: &HashMap<String, &'a str>, key: &str) -> Result<&'a str> {
    // get string
    let an = get_str(data, key)?;

    // check if alphanumeric
    if an.is_empty() || !is_alphanumeric(an) {
        return Fail::from(format!("{} is not alphanumeric", key));
    }

    // return string
    Ok(an)
}

/// Get username string and check if alphanumeric
pub fn get_username<'a>(data: &HashMap<String, &'a str>) -> Result<&'a str> {
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

/// User login/token management
#[derive(Clone, Debug)]
pub struct UserLogins {
    valid_login: u64,
    logins: HashMap<String, Vec<(String, SystemTime)>>,
}

impl UserLogins {
    /// Create empty
    pub fn new(valid_login: u64) -> Self {
        Self {
            valid_login,
            logins: HashMap::new(),
        }
    }

    /// Check if login token is valid
    pub fn valid(&self, user: &str, token: &str) -> bool {
        // get logins
        match self.logins.get(user) {
            Some(logins) => {
                // check login
                logins
                    .iter()
                    .any(|login| login.0 == token && check_unexpired(&login.1, self.valid_login))
            }
            None => false,
        }
    }

    /// Generate login token for user
    pub fn add(&mut self, user: &str) -> &str {
        // generate random token and get logins
        let token = random_an(64);
        match self.logins.get_mut(user) {
            Some(logins) => {
                // remove expired logins and return logins
                Self::remove_expired(logins, self.valid_login);
                logins.push((token, SystemTime::now()));
            }
            None => {
                // create new logins vector for user
                self.logins
                    .insert(user.to_string(), [(token, SystemTime::now())].to_vec());
            }
        };

        // return token
        &self.logins[user].last().unwrap().0
    }

    /// Remove login token for user
    pub fn remove(&mut self, user: &str, token: &str) {
        // get logins
        if let Some(logins) = self.logins.get_mut(user) {
            // remove token
            let valid_login = self.valid_login;
            logins.retain(|login| login.0 != token && check_unexpired(&login.1, valid_login));
        }
    }

    /// Remove all logins for user
    pub fn remove_user(&mut self, user: &str) {
        // remove user
        self.logins.remove(user);
    }

    /// Rename user entry
    pub fn rename(&mut self, user: &str, new_user: String) {
        if let Some(logins) = self.logins.remove(user) {
            self.logins.insert(new_user, logins);
        }
    }

    /// Remove expired logins
    fn remove_expired(logins: &mut Vec<(String, SystemTime)>, valid_login: u64) {
        if logins.len() > 100 {
            logins.drain(0..(logins.len() - 100));
        }
        (*logins).retain(|login| check_unexpired(&login.1, valid_login));
    }
}

/// User storage files
#[derive(Debug)]
pub struct UserFiles {
    data_dir: String,
    files: HashMap<String, RwLock<StorageFile>>,
}

impl UserFiles {
    /// New user files instance
    pub fn new(data_dir: String) -> Self {
        Self {
            data_dir,
            files: HashMap::new(),
        }
    }

    /// Check if user file exists
    pub fn exists(&self, name: &str) -> bool {
        self.files.contains_key(name)
    }

    /// Create storage file
    pub fn create(&mut self, name: &str) -> Result<()> {
        // check if alphanumeric
        if name.is_empty() || !is_alphanumeric(name) {
            return Fail::from("name not alphanumeric");
        }

        // create and add to map
        let storage = RwLock::new(StorageFile::new(format!("{}/{}.edb", self.data_dir, name))?);
        self.files.insert(name.to_string(), storage);
        Ok(())
    }

    /// Rename storage file
    pub fn rename(&mut self, name: &str, new_name: &str) -> Result<()> {
        // check if already exists
        if self.exists(new_name) {
            return Fail::from("name already exists");
        }

        // create new, copy contents and delete old
        self.create(new_name)?;
        self.write(new_name)?
            .raw_write(self.read(name)?.raw().to_string())?;
        self.delete(name)
    }

    /// Read access on existing storage file
    pub fn read(&self, name: &str) -> Result<RwLockReadGuard<StorageFile>> {
        // check if alphanumeric
        if name.is_empty() || !is_alphanumeric(name) {
            return Fail::from("name not alphanumeric");
        }

        // return storage file or fail if not exists
        match self.files.get(name) {
            Some(storage) => Ok(storage.read().unwrap()),
            None => Fail::from("storage does not exist"),
        }
    }

    /// Write access on storage file (create if not existent)
    pub fn write(&self, name: &str) -> Result<RwLockWriteGuard<StorageFile>> {
        // check if alphanumeric
        if name.is_empty() || !is_alphanumeric(name) {
            return Fail::from("name not alphanumeric");
        }

        // return storage file or fail if not exists
        match self.files.get(name) {
            Some(storage) => Ok(storage.write().unwrap()),
            None => Fail::from("storage does not exist"),
        }
    }

    /// Delete storage file
    pub fn delete(&mut self, name: &str) -> Result<()> {
        // check if alphanumeric
        if name.is_empty() || !is_alphanumeric(name) {
            return Fail::from("name not alphanumeric");
        }

        // delete storage file
        self.files.remove(name);
        delete_file(format!("{}/{}.edb", self.data_dir, name))
    }
}

/// API handler action
pub enum ApiAction {
    Simple,
    Register,
}

/// Security management (rate limiting, banning)
#[derive(Debug)]
pub struct SecurityManager {
    bans: RwLock<HashMap<String, SystemTime>>,
    fail_counter: RwLock<HashMap<String, (u32, SystemTime)>>,
    register_counter: RwLock<HashMap<String, (u32, SystemTime)>>,
    ban_time: u64,
    login_fails: u32,
    login_time: u64,
    account_limit: u32,
    log: bool,
}

impl SecurityManager {
    /// Create empty
    pub fn new(
        ban_time: u64,
        login_fails: u32,
        login_time: u64,
        account_limit: u32,
        log: bool,
    ) -> Self {
        Self {
            bans: RwLock::new(HashMap::new()),
            fail_counter: RwLock::new(HashMap::new()),
            register_counter: RwLock::new(HashMap::new()),
            ban_time,
            login_fails,
            login_time,
            account_limit,
            log,
        }
    }

    /// Check action
    pub fn check(&self, ip: impl AsRef<str>, action: ApiAction) -> bool {
        let ip = ip.as_ref();

        // check ban
        let banned = match self.bans.read().unwrap().get(ip) {
            Some(ban) => {
                // check if ban is not expired
                check_unexpired(ban, self.ban_time)
            }
            None => false,
        };

        match action {
            ApiAction::Register => {
                // check account registrations
                match self.register_counter.read().unwrap().get(ip) {
                    Some(registrations) => {
                        (registrations.0 <= self.account_limit
                            || !check_unexpired(&registrations.1, 3600))
                            && !banned
                    }
                    None => !banned,
                }
            }
            _ => {
                // check login fails
                match self.fail_counter.read().unwrap().get(ip) {
                    Some(fails) => {
                        (fails.0 <= self.login_fails || !check_unexpired(&fails.1, self.login_time))
                            && !banned
                    }
                    None => !banned,
                }
            }
        }
    }

    pub fn login_fail(&mut self, ip: impl AsRef<str>) {
        let ip = ip.as_ref();
        let mut fail_counter = self.fail_counter.write().unwrap();

        // cleanup if too many IP addresses
        if fail_counter.len() > 100_000 {
            return fail_counter.clear();
        }

        // add to counter
        if let Some(count) = fail_counter.get_mut(ip) {
            // check if expired
            if !check_unexpired(&count.1, self.login_time) {
                // reset counter
                count.0 = 1;
                count.1 = SystemTime::now();
            } else {
                // add to existing counter
                count.0 += 1;

                // check if ban necessary
                if count.0 > self.login_fails {
                    self.bans
                        .write()
                        .unwrap()
                        .insert(ip.to_string(), SystemTime::now());
                    if self.log {
                        println!("Banned IP address: {}", ip);
                    }
                }
            }
        } else {
            // create new counter
            fail_counter.insert(ip.to_string(), (1, SystemTime::now()));
        }
    }

    pub fn registration(&mut self, ip: impl AsRef<str>) {
        let ip = ip.as_ref();
        let mut register_counter = self.register_counter.write().unwrap();

        // cleanup if too many IP addresses
        if register_counter.len() > 100_000 {
            return register_counter.clear();
        }

        // add to counter
        if let Some(count) = register_counter.get_mut(ip) {
            // check if expired
            if !check_unexpired(&count.1, 3600) {
                // reset counter
                count.0 = 1;
                count.1 = SystemTime::now();
            } else {
                // add to existing counter
                count.0 += 1;
            }
        } else {
            // create new counter
            register_counter.insert(ip.to_string(), (1, SystemTime::now()));
        }
    }

    /*
    /// Check if login token is valid and remove expired
    pub fn valid(&self, user: &str, token: &str) -> bool {
        // get logins
        match self.logins.get(user) {
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
        let token = random_an(64);
        match self.logins.get_mut(user) {
            Some(logins) => {
                // remove expired logins and return logins
                Self::remove_expired(logins);
                logins.push((token, SystemTime::now()));
            }
            None => {
                // create new logins vector for user
                self.logins
                    .insert(user.to_string(), [(token, SystemTime::now())].to_vec());
            }
        };

        // return token
        &self.logins[user].last().unwrap().0
    }

    /// Remove login token for user
    pub fn remove(&mut self, user: &str, token: &str) {
        // get logins
        if let Some(logins) = self.logins.get_mut(user) {
            // remove token
            logins.retain(|login| login.0 != token && Self::check_unexpired(&login.1));
        }
    }

    /// Remove all logins for user
    pub fn remove_user(&mut self, user: &str) {
        // remove user
        self.logins.remove(user);
    }

    /// Rename user entry
    pub fn rename(&mut self, user: &str, new_user: String) {
        if let Some(logins) = self.logins.remove(user) {
            self.logins.insert(new_user, logins);
        }
    }

    /// Remove expired logins
    fn remove_expired(logins: &mut Vec<(String, SystemTime)>) {
        (*logins).retain(|login| Self::check_unexpired(&login.1));
    }
    */
}

/// Check if is not expired
fn check_unexpired(expiration: &SystemTime, time: u64) -> bool {
    expiration
        .elapsed()
        .unwrap_or_else(|_| Duration::from_secs(u64::max_value()))
        .as_secs()
        < time
}
