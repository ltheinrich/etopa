//! Commons

pub use crate::utils::*;

use crate::api::user::UserLogins;
use etopa::data::StorageFile;

/// Help output
pub const HELP: &str = "Help: TODO";

/// Cargo.toml
pub const CARGO_TOML: &str = include_str!("../Cargo.toml");

/// Data shared between handlers
#[derive(Debug)]
pub struct SharedData {
    pub user_data: StorageFile,
    pub user_logins: UserLogins,
    pub data_dir: String,
}

impl SharedData {
    /// Default SharedData
    pub fn new(user_data: StorageFile, data_dir: String) -> Self {
        // return default with provided user data
        Self {
            user_data,
            user_logins: UserLogins::new(),
            data_dir,
        }
    }
}
