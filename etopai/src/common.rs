//! Commons

pub use crate::utils::*;

use crate::api::user::UserLogins;
use etopa::data::StorageFile;

/// Data shared between handlers
#[derive(Debug)]
pub struct SharedData {
    pub user_data: StorageFile,
    pub used_files: Vec<String>,
    pub user_logins: UserLogins,
    pub data_dir: String,
}

impl SharedData {
    /// Default SharedData
    pub fn new(user_data: StorageFile, data_dir: String) -> Self {
        // return default with provided user data
        Self {
            user_data,
            used_files: Vec::new(),
            user_logins: UserLogins::new(),
            data_dir,
        }
    }
}
