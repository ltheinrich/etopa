//! Cryptography utils

use hex::encode;
use sha3::{Digest, Sha3_256};

/// Generate password hash for API usage -> sha3-256(etopa_ + sha3-256(password))
pub fn hash_password(password: &[u8]) -> String {
    // init hasher and hash password
    let mut hasher = Sha3_256::new();
    hasher.input(password);
    let enc = encode(hasher.result());

    // hash the hash
    hasher = Sha3_256::new();
    hasher.input(b"etopa_");
    hasher.input(enc);
    let result = hasher.result();

    // return hex encoded
    encode(result)
}
