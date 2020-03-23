//! Database

pub trait Storage<'a> {}

// Encrypted storage
pub struct SecureStorage<'a> {
    file: &'a str,
}
