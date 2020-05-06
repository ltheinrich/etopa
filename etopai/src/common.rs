//! Commons

pub use crate::utils::*;

use crate::data::StorageFile;
use crate::utils::UserLogins;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

/// Help output
pub const HELP: &str = "
Usage: etopai [OPTIONS]
String S, Integer I, Boolean B

Options:
  --port       I       Port (4490)
  --addr       S       IP address ([::])
  --threads    I       Number of threads to start (2)
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
    pub data_dir: String,
}

impl SharedData {
    /// Default SharedData
    pub fn new(users: StorageFile, data_dir: String) -> Self {
        // return default with provided users storage
        Self {
            users: RwLock::new(users),
            logins: RwLock::new(UserLogins::new()),
            data_dir,
        }
    }

    /// Users database read-only
    pub fn users(&self) -> RwLockReadGuard<'_, StorageFile> {
        self.users.read().unwrap()
    }

    /// Users database writeable
    pub fn users_mut(&self) -> RwLockWriteGuard<'_, StorageFile> {
        self.users.write().unwrap()
    }

    /// User logins read-only
    pub fn logins(&self) -> RwLockReadGuard<'_, UserLogins> {
        self.logins.read().unwrap()
    }

    /// User logins writeable
    pub fn logins_mut(&self) -> RwLockWriteGuard<'_, UserLogins> {
        self.logins.write().unwrap()
    }

    /// Data directory read-only
    pub fn data_dir(&self) -> &str {
        &self.data_dir
    }
}
