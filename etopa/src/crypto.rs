//! Cryptography utils

use aes_gcm::{
    aead::{generic_array::GenericArray, Aead, NewAead},
    Aes256Gcm,
};
use argon2::{
    hash_encoded, verify_encoded, Config as Argon2Config, ThreadMode as Argon2ThreadMode,
};
pub use hex::{decode as hex_decode, encode as hex_encode};
use kern::Fail;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sha3::{Digest, Sha3_256};

/// Generate password hash for API usage -> sha3-256(etopa + sha3-256(username + sha3-256(password))
pub fn hash_password(password: impl AsRef<[u8]>, username: impl AsRef<[u8]>) -> String {
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

/// Generate secret name hash for API usage -> sha3-256(etopa_secret + sha3-256(username + sha3-256(name))
pub fn hash_name(name: impl AsRef<[u8]>, username: impl AsRef<[u8]>) -> String {
    // init hasher and hash name
    let mut hasher = Sha3_256::new();
    hasher.input(name);
    let mut enc = hex_encode(hasher.result());

    // hash the hash with username
    hasher = Sha3_256::new();
    hasher.input(username);
    hasher.input(enc);
    enc = hex_encode(hasher.result());

    // hash the hash with etopa
    hasher = Sha3_256::new();
    hasher.input(b"etopa_secret");
    hasher.input(enc);
    let result = hasher.result();

    // return hex encoded
    hex_encode(result)
}

/// Generate key (password hash) for secure storage encryption
///
/// sha3-256(username + sha3-256(secure_storage + sha3-256(password))
/// Even username length -> first 32 bytes
/// Uneven username length -> last 32 bytes
pub fn hash_key(password: impl AsRef<[u8]>, username: impl AsRef<[u8]>) -> String {
    // as ref
    let username = username.as_ref();

    // init hasher and hash password
    let mut hasher = Sha3_256::new();
    hasher.input(password);
    let mut enc = hex_encode(hasher.result());

    // hash the hash with username
    hasher = Sha3_256::new();
    hasher.input(b"secure_storage");
    hasher.input(enc);
    enc = hex_encode(hasher.result());

    // hash the hash with etopa
    hasher = Sha3_256::new();
    hasher.input(username);
    hasher.input(enc);
    let result = hasher.result();

    // hex encode and check uneven username length
    let mut full_key = hex_encode(result);
    if username.len() % 2 != 0 {
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
    hasher.input(plaintext);
    hex_encode(hasher.result())
}

/// Intialize Aes256Gcm with custom key
fn init_aes(raw_key: impl AsRef<[u8]>) -> Aes256Gcm {
    // initialize aes with key
    let key = GenericArray::clone_from_slice(raw_key.as_ref());
    Aes256Gcm::new(key)
}

/// Decrypt secure storage
pub fn decrypt(raw_data: impl AsRef<[u8]>, raw_key: impl AsRef<[u8]>) -> Result<String, Fail> {
    // init
    let raw_data = raw_data.as_ref();
    let aead = init_aes(raw_key);

    // check if contains at least nonce (first 12 bytes)
    if raw_data.len() < 13 {
        // no data
        Ok("{}".to_string())
    } else {
        // get nonce and decrypt data
        let nonce = GenericArray::clone_from_slice(&raw_data[..12]);
        let decrypted = aead
            .decrypt(&nonce, &raw_data[12..])
            .or_else(|_| Fail::from("could not decrypt secure storage data"))?;

        // decrypted to string
        let data = String::from_utf8(decrypted).or_else(Fail::from)?;
        Ok(data)
    }
}

/// Encrypt secure storage
pub fn encrypt(data: impl AsRef<[u8]>, raw_key: impl AsRef<[u8]>) -> Result<Vec<u8>, Fail> {
    // init
    let aead = init_aes(raw_key);
    let mut rng = thread_rng();

    // generate random nonce
    let mut raw_data: Vec<u8> = (0..12).map(|_| rng.gen()).collect();
    let nonce = GenericArray::clone_from_slice(&raw_data);

    // encrypt data
    let mut encrypted = aead
        .encrypt(&nonce, data.as_ref())
        .or_else(|_| Fail::from("could not encrypt secure storage data"))?;

    // add encrypted and return
    raw_data.append(&mut encrypted);
    Ok(raw_data)
}

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
