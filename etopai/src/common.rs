//! Commons

pub use crate::utils::*;

use crate::data::StorageFile;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

/// Help output
pub const HELP: &str = "
Usage: etopai [OPTIONS]
String S, Integer I, Boolean B

Options:
  --port       I       Port (4490)
  --addr       S       IP address ([::])
  --threads    I       Number of threads to start (2)
  --vlt        I       Valid login time in seconds (604800/1 week)
  --data       S       Data directory (data)
  --cert       S       Path to TLS certificate (DATA_DIR/cert.pem)
  --key        S       Path to TLS certificate key (DATA_DIR/key.pem)";

/// Cargo.toml
pub const CARGO_TOML: &str = include_str!("../Cargo.toml");

/// Data shared between handlers
#[derive(Debug)]
pub struct SharedData {
    pub users: RwLock<StorageFile>,
    pub logins: RwLock<UserLogins>,
    pub files: RwLock<UserFiles>,
    pub data_dir: String,
}

impl SharedData {
    /// Default SharedData
    pub fn new(users: StorageFile, valid_login: u64, data_dir: String) -> Self {
        // return default with provided users storage
        Self {
            users: RwLock::new(users),
            logins: RwLock::new(UserLogins::new(valid_login)),
            files: RwLock::new(UserFiles::new(data_dir.clone())),
            data_dir,
        }
    }

    /// Users database read-only
    pub fn users(&self) -> RwLockReadGuard<StorageFile> {
        self.users.read().unwrap()
    }

    /// Users database writeable
    pub fn users_mut(&self) -> RwLockWriteGuard<StorageFile> {
        self.users.write().unwrap()
    }

    /// User logins read-only
    pub fn logins(&self) -> RwLockReadGuard<UserLogins> {
        self.logins.read().unwrap()
    }

    /// User logins writeable
    pub fn logins_mut(&self) -> RwLockWriteGuard<UserLogins> {
        self.logins.write().unwrap()
    }

    /// User files read-only
    pub fn files(&self) -> RwLockReadGuard<UserFiles> {
        self.files.read().unwrap()
    }

    /// User files writeable
    pub fn files_mut(&self) -> RwLockWriteGuard<UserFiles> {
        self.files.write().unwrap()
    }

    /*
    /// Data directory read-only
    pub fn data_dir(&self) -> &str {
        &self.data_dir
    }
    */
}
