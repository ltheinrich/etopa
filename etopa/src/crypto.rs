//! Cryptography utils

use hex::encode;
use sha3::{Digest, Sha3_256};

/// Generate password hash for API usage -> sha3-256(etopa + sha3-256(username + sha3-256(password))
pub fn hash_password(password: &[u8], username: &[u8]) -> String {
    // init hasher and hash password
    let mut hasher = Sha3_256::new();
    hasher.input(password);
    let mut enc = encode(hasher.result());

    // hash the hash with username
    hasher = Sha3_256::new();
    hasher.input(username);
    hasher.input(enc);
    enc = encode(hasher.result());

    // hash the hash with etopa
    hasher = Sha3_256::new();
    hasher.input(b"etopa");
    hasher.input(enc);
    let result = hasher.result();

    // return hex encoded
    encode(result)
}
