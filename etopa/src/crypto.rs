//! Cryptography utils

pub use hex::{decode as hex_decode, encode as hex_encode};

use kern::Fail;
use sha3::{Digest, Sha3_256};

/// Generate password hash for API usage -> sha3-256(etopa + sha3-256(username + sha3-256(password))
pub fn hash_password(password: &[u8], username: &[u8]) -> String {
    // init hasher and hash password
    let mut hasher = Sha3_256::new();
    hasher.input(password);
    let mut enc = hex_encode(hasher.result());

    // hash the hash with username
    hasher = Sha3_256::new();
    hasher.input(username);
    hasher.input(enc);
    enc = hex_encode(hasher.result());

    // hash the hash with etopa
    hasher = Sha3_256::new();
    hasher.input(b"etopa");
    hasher.input(enc);
    let result = hasher.result();

    // return hex encoded
    hex_encode(result)
}

use argon2::{
    hash_encoded, verify_encoded, Config as Argon2Config, ThreadMode as Argon2ThreadMode,
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};

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

/// Generate random alphanumeric string
pub fn random_an(len: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric).take(len).collect()
}
