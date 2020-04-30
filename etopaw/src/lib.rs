//! Etopa for Web
#![cfg(target_arch = "wasm32")]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[macro_use]
extern crate serde_derive;

use etopa::{
    crypto,
    data::{parse, serialize},
    totp::Generator,
    wasm_bindgen::{self, prelude::*},
};
use std::collections::BTreeMap;

/// Better panic messages
#[wasm_bindgen]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

/// Hash password for API usage
#[wasm_bindgen]
pub fn hash_password(password: &str, username: &str) -> String {
    crypto::hash_password(password.as_bytes(), username.as_bytes())
}

/// Hash password for encryption
#[wasm_bindgen]
pub fn hash_key(password: &str, username: &str) -> String {
    crypto::hash_key(password.as_bytes(), username.as_bytes())
}

/// Hash secret name
#[wasm_bindgen]
pub fn hash_name(name: &str, username: &str) -> String {
    crypto::hash_name(name.as_bytes(), username.as_bytes())
}

/// Decrypt from hex
#[wasm_bindgen]
pub fn decrypt_hex(data: &str, key: &str) -> String {
    // decode hex
    let dec = crypto::hex_decode(data).unwrap();

    // return decrypted
    crypto::decrypt(dec, key).unwrap_or("FAILED TO DECRYPT".to_string())
}

/// Encrypt to hex
#[wasm_bindgen]
pub fn encrypt_hex(data: &str, key: &str) -> String {
    // encrypt
    let enc = crypto::encrypt(data, key).unwrap();

    // return hex-encoded
    crypto::hex_encode(enc)
}

#[wasm_bindgen]
pub fn gen_token(secret: &str, time_millis: u64) -> String {
    match Generator::new(secret) {
        Ok(gen) => gen
            .token_at(time_millis / 1000)
            .unwrap_or("INVALID SECRET".to_string()),
        Err(err) => err.to_string(),
    }
}

#[derive(Serialize, Deserialize)]
struct StringMap(BTreeMap<String, String>);

/// Parse storage file and decrypt secrets
#[wasm_bindgen]
pub fn parse_storage(mut data: Vec<u8>, key: &str) -> JsValue {
    // remove \r\n
    if data.len() > 1 {
        data.truncate(data.len() - 2);
    }

    // parse and iterate through entries
    let mut map = parse(data).unwrap();
    map.iter_mut().for_each(|(k, v)| {
        // check if secret or secret name
        if k.ends_with("_secret") || k.ends_with("_secret_name") {
            // decode hex
            let dec = crypto::hex_decode(&v).unwrap();

            // decrypt secret and modify
            *v = crypto::decrypt(dec, key).unwrap_or("FAILED TO DECRYPT".to_string())
        }
    });

    // return to JS
    JsValue::from_serde(&StringMap(map)).unwrap()
}

/// Encrypt secrets and serialize map
#[wasm_bindgen]
pub fn serialize_storage(storage: JsValue, key: &str, username: &str) -> String {
    // deserialize from JS
    let storage: StringMap = storage.into_serde().unwrap();

    // new map and iterate through storage entries
    let mut map = BTreeMap::new();
    for (k, v) in storage.0 {
        // hash secret name
        let name = crypto::hash_name(&k, username);

        // encrypt secret and name
        let enc_secret = crypto::encrypt(&v, key).unwrap();
        let enc_name = crypto::encrypt(&k, key).unwrap();

        // hex encode secret and name
        let hex_secret = crypto::hex_encode(enc_secret);
        let hex_name = crypto::hex_encode(enc_name);

        // add secret and secret name
        map.insert(format!("{}_secret", name), hex_secret);
        map.insert(format!("{}_secret_name", name), hex_name);
    }

    // return serialized
    serialize(&map).unwrap()
}
