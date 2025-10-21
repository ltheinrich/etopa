//! Cryptography utils

use aes_gcm::{
    Aes256Gcm,
    aead::{Aead, KeyInit},
};
use argon2::{Config, ThreadMode, Variant, Version, hash_encoded, verify_encoded};
pub use hex::{decode as hex_decode, encode as hex_encode};
use kern::{Fail, Result};
use rand::{Rng, distr::Alphanumeric, rng};
use sha3::{Digest, Sha3_256};

use crate::common::Nonce;

/// Generate password hash for API usage -> sha3-256(etopa + sha3-256(password))
pub fn hash_password(password: impl AsRef<[u8]>) -> String {
    // init hasher and hash password
    let mut hasher = Sha3_256::new();
    hasher.update(password);
    let enc = hex_encode(hasher.finalize());

    // hash the hash with etopa
    hasher = Sha3_256::new();
    hasher.update(b"etopa");
    hasher.update(enc);
    let result = hasher.finalize();

    // return hex encoded
    hex_encode(result)
}

/// Generate pin hash for app encryption (local data)
///
/// sha3-256(sha3-256(etopan + sha3-256(etopa_app_pin + sha3-256(pin)))
/// Even PIN length -> first 32 bytes
/// Uneven PIN length -> last 32 bytes
pub fn hash_pin(pin: impl AsRef<[u8]>) -> String {
    // as ref
    let pin = pin.as_ref();

    // init hasher and hash pin
    let mut hasher = Sha3_256::new();
    hasher.update(pin);
    let enc = hex_encode(hasher.finalize());

    // hash the hash with etopa_app_pin
    hasher = Sha3_256::new();
    hasher.update(b"etopa_app_pin");
    hasher.update(enc);
    let enc = hasher.finalize();

    // hash the hash with etopan
    hasher = Sha3_256::new();
    hasher.update(b"etopan");
    hasher.update(enc);
    let enc = hasher.finalize();

    // hash the hash
    hasher = Sha3_256::new();
    hasher.update(enc);
    let result = hasher.finalize();

    // hex encode and check uneven pin length
    let mut full_key = hex_encode(result);
    if pin.len() % 2 != 0 {
        // return last 32 bytes of key
        return full_key.split_at(32).1.to_string();
    }

    // return first 32 bytes of key
    full_key.truncate(32);
    full_key
}

/// Generate secret name hash for API usage -> sha3-256(etopa_secret + sha3-256(name))
pub fn hash_name(name: impl AsRef<[u8]>) -> String {
    // init hasher and hash name
    let mut hasher = Sha3_256::new();
    hasher.update(name);
    let enc = hex_encode(hasher.finalize());

    // hash the hash with etopa_Secret
    hasher = Sha3_256::new();
    hasher.update(b"etopa_secret");
    hasher.update(enc);
    let result = hasher.finalize();

    // return hex encoded
    hex_encode(result)
}

/// Generate key (password hash) for secure storage encryption
///
/// sha3-256(secure_storage + sha3-256(password))
/// Even password length -> first 32 bytes
/// Uneven password length -> last 32 bytes
pub fn hash_key(password: impl AsRef<[u8]>) -> String {
    // as ref
    let password = password.as_ref();

    // init hasher and hash password
    let mut hasher = Sha3_256::new();
    hasher.update(password);
    let enc = hex_encode(hasher.finalize());

    // hash the hash with secure_storage
    hasher = Sha3_256::new();
    hasher.update(b"secure_storage");
    hasher.update(enc);
    let result = hasher.finalize();

    // hex encode and check uneven password length
    let mut full_key = hex_encode(result);
    if password.len() % 2 != 0 {
        // return last 32 bytes of key
        return full_key.split_at(32).1.to_string();
    }

    // return first 32 bytes of key
    full_key.truncate(32);
    full_key
}

/// Generate sha3-256 hash
pub fn hash(plaintext: impl AsRef<[u8]>) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(plaintext);
    hex_encode(hasher.finalize())
}

/// Intialize Aes256Gcm with custom key
fn init_aes(raw_key: impl AsRef<[u8]>) -> Result<Aes256Gcm> {
    // initialize aes with key
    //let key = GenericArray::clone_from_slice(raw_key.as_ref());
    let key = raw_key.as_ref().try_into()?;
    Ok(Aes256Gcm::new(&key))
}

/// Decrypt secure storage
pub fn decrypt(raw_data: impl AsRef<[u8]>, raw_key: impl AsRef<[u8]>) -> Result<String> {
    // init
    let raw_data = raw_data.as_ref();
    let aead = init_aes(raw_key)?;

    // check if contains at least nonce (first 12 bytes)
    if raw_data.len() < 13 {
        // no data
        Ok("{}".to_string())
    } else {
        // get nonce and decrypt data
        let nonce = &raw_data[..12].try_into()?;
        let decrypted = aead
            .decrypt(nonce, &raw_data[12..])
            .or_else(|_| Fail::from("could not decrypt secure storage data"))?;

        // decrypted to string
        let data = String::from_utf8(decrypted).or_else(Fail::from)?;
        Ok(data)
    }
}

/// Encrypt secure storage
pub fn encrypt(data: impl AsRef<[u8]>, raw_key: impl AsRef<[u8]>) -> Result<Vec<u8>> {
    // init
    let aead = init_aes(raw_key)?;
    let mut rng = rng();

    // generate random nonce
    let mut raw_data: Vec<u8> = (0..12).map(|_| rng.random()).collect();
    let nonce: Nonce = raw_data.as_slice().try_into()?;

    // encrypt data
    let mut encrypted = aead
        .encrypt(&nonce, data.as_ref())
        .or_else(|_| Fail::from("could not encrypt secure storage data"))?;

    // add encrypted and return
    raw_data.append(&mut encrypted);
    Ok(raw_data)
}

/// Generate Argon2 password hash
pub fn argon2_hash(pwd: impl AsRef<[u8]>, salt: impl AsRef<[u8]>) -> Result<String> {
    let config = Config {
        variant: Variant::Argon2id,
        // original config ..Config::original()
        ad: &[],
        hash_length: 32,
        lanes: 1,
        mem_cost: 4096,
        secret: &[],
        time_cost: 3,
        version: Version::Version13,
        thread_mode: ThreadMode::Sequential,
    };
    hash_encoded(pwd.as_ref(), salt.as_ref(), &config).or_else(Fail::from)
}

/// Verify Argon2 password hash
pub fn argon2_verify(encoded: impl AsRef<str>, pwd: impl AsRef<[u8]>) -> bool {
    verify_encoded(encoded.as_ref(), pwd.as_ref()).unwrap_or(false)
}

/// Generate random vector
pub fn random(size: usize) -> Vec<u8> {
    let mut rng = rng();
    (0..size).map(|_| rng.random()).collect()
}

/// Generate random alphanumeric string
pub fn random_an(len: usize) -> String {
    let rand_an = rng().sample_iter(&Alphanumeric).take(len).collect();
    String::from_utf8(rand_an).unwrap()
}
