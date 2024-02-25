//! Commons

pub use crate::utils::*;

use etopa::data::StorageFile;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

/// Help output
pub const HELP: &str = "Usage: etopai [OPTIONS]
String S, Integer I, Boolean B

Options:
  --port       I       Port (4490)
  --addr       S       IP address ([::])
  --threads    I       Number of threads to start (2)
  --vlt        I       Valid login time in seconds (604800/1 week)
  --bantime    I       IP address ban time in seconds (3600/1 hour)
  --loginfails I       Login fails until ban (50)
  --logintime  I       Login fails cleanup time in seconds (60)
  --acclimit   I       Account registration limit per hour (10)
  --data       S       Data directory (data)
  --cert       S       Path to TLS certificate (DATA_DIR/cert.pem)
  --key        S       Path to TLS certificate key (DATA_DIR/key.pem)
  --licenses           List licenses of project and libraries
  --nolog              Print no log messages";

/// licenses
pub const LICENSES: &str = include_str!("../../NOTICE.txt");

/// Android build.gradle (get version string)
pub const BUILD_GRADLE: &str = include_str!("../../etopan-app/app/build.gradle");

/// Data shared between handlers
#[derive(Debug)]
pub struct SharedData {
    pub users: RwLock<StorageFile>,
    pub logins: RwLock<UserLogins>,
    pub files: RwLock<UserFiles>,
    pub security: RwLock<SecurityManager>,
    pub data_dir: String,
    pub log: bool,
}

impl SharedData {
    /// Default SharedData
    pub fn new(
        users: StorageFile,
        security: SecurityManager,
        valid_login: u64,
        data_dir: String,
        log: bool,
    ) -> Self {
        // return default with provided users storage
        Self {
            users: RwLock::new(users),
            logins: RwLock::new(UserLogins::new(valid_login)),
            files: RwLock::new(UserFiles::new(data_dir.clone())),
            security: RwLock::new(security),
            data_dir,
            log,
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

    /// Security manager read-only
    pub fn security(&self) -> RwLockReadGuard<SecurityManager> {
        self.security.read().unwrap()
    }

    /// Security manager writeable
    pub fn security_mut(&self) -> RwLockWriteGuard<SecurityManager> {
        self.security.write().unwrap()
    }

    /*
    /// Get print log messages setting
    pub fn log(&self) -> bool {
        self.log
    }
    */

    /*
    /// Data directory read-only
    pub fn data_dir(&self) -> &str {
        &self.data_dir
    }
    */
}
