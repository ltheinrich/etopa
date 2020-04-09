//! Etopa library

pub mod common;
pub mod crypto;
pub mod data;
pub mod totp;

pub use hex::{decode as hex_decode, encode as hex_encode};
pub use kern::*;

use argon2::{
    hash_encoded, verify_encoded, Config as Argon2Config, ThreadMode as Argon2ThreadMode,
};
use rand::{thread_rng, Rng};

/// Generate Argon2 password hash
pub fn argon2_hash(pwd: &[u8], salt: &[u8]) -> Result<String, Fail> {
    let mut config = Argon2Config::default();
    config.lanes = 3;
    config.thread_mode = Argon2ThreadMode::Sequential;
    config.mem_cost = 1024;
    config.time_cost = 2;
    hash_encoded(pwd, salt, &config).or_else(Fail::from)
}

/// Verify Argon2 password hash
pub fn argon2_verify(encoded: &str, pwd: &[u8]) -> bool {
    verify_encoded(encoded, pwd).unwrap_or(false)
}

/// Generate random vector
pub fn random(size: usize) -> Vec<u8> {
    let mut rng = thread_rng();
    (0..size).map(|_| rng.gen()).collect()
}
