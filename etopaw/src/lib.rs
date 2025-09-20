//! Etopa for Web
//#![cfg(target_family = "wasm")] // TODO: temporary fix for error: unneeded unit expression

#[macro_use]
extern crate serde_derive;

use etopa::{
    crypto,
    data::{parse, serialize},
    totp::Generator,
};
use serde_wasm_bindgen::{from_value, to_value};
use std::collections::HashMap;
use wasm_bindgen::{self, prelude::*};

#[cfg(test)]
use wasm_bindgen_test::wasm_bindgen_test;

/// Better panic messages
#[wasm_bindgen]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

/// Hash password for API usage
#[wasm_bindgen]
pub fn hash_password(password: &str) -> String {
    crypto::hash_password(password.as_bytes())
}

/// Hash password for encryption
#[wasm_bindgen]
pub fn hash_key(password: &str) -> String {
    crypto::hash_key(password.as_bytes())
}

/// Hash secret name
#[wasm_bindgen]
pub fn hash_name(name: &str) -> String {
    crypto::hash_name(name.as_bytes())
}

/// Argon2 password hash
#[wasm_bindgen]
pub fn argon2_hash(password: &str) -> String {
    let salt = crypto::random(16);
    let password_hash = crypto::hash_password(password);
    crypto::argon2_hash(password_hash, salt).unwrap()
}

/// Decrypt from hex
#[wasm_bindgen]
pub fn decrypt_hex(data: &str, key: &str) -> String {
    // decode hex
    let dec = crypto::hex_decode(data).unwrap();

    // return decrypted
    crypto::decrypt(dec, key).unwrap()
}

/// Encrypt to hex
#[wasm_bindgen]
pub fn encrypt_hex(data: &str, key: &str) -> String {
    // encrypt
    let enc = crypto::encrypt(data, key).unwrap();

    // return hex-encoded
    crypto::hex_encode(enc)
}

/// Decode from hex
#[wasm_bindgen]
pub fn hex_decode(data: &str) -> Vec<u8> {
    // decode from hex
    crypto::hex_decode(data).unwrap()
}

/// Encode to hex
#[wasm_bindgen]
pub fn hex_encode(data: &[u8]) -> String {
    // encode to hex
    crypto::hex_encode(data)
}

#[wasm_bindgen]
pub fn gen_token(secret: &str, time_millis: u64) -> String {
    match Generator::new(secret) {
        Ok(generator) => generator.token_at(time_millis / 1000),
        _ => "invalid secret".to_string(),
    }
}

#[test]
#[wasm_bindgen_test]
fn test_gen_token() {
    let token1 = gen_token("JBSWY3DPEHPK3PXP", 1737289933123);
    assert_eq!("880121", token1);

    let token2 = gen_token("mr6FAijp7noNGd3f4iZZfnUHi5MF2mts", 1737290803000);
    assert_eq!("721002", token2);

    let token3 = gen_token("C7G3JBj2hO", 1737290921789);
    assert_eq!("957794", token3);

    let token4 = gen_token("nysy2x64es", 1737290964999);
    assert_eq!("971040", token4);

    let token5 = gen_token("QWERTZUIOPASDFGHJKLYXCVBNM234567", 1737291065111);
    assert_eq!("505744", token5);
}

#[derive(Serialize, Deserialize)]
struct StringMap(HashMap<String, String>);

//#[derive(Serialize, Deserialize)]
//struct StringVec(Vec<String>);

/// Unsorted parse storage file and decrypt secrets
#[wasm_bindgen]
pub fn parse_storage(mut data: Vec<u8>, key: &str) -> JsValue {
    // remove \r\n
    if data.len() > 1 {
        data.truncate(data.len() - 2);
    }

    // parse and iterate through entries
    let data = String::from_utf8(data).unwrap();
    let mut map = parse(&data);
    map.iter_mut().for_each(|(k, v)| {
        // check if secret or secret name
        if k.ends_with("_secret") || k.ends_with("_secret_name") || k == "secrets_sort" {
            // decode hex
            let dec = crypto::hex_decode(v.as_bytes()).unwrap();

            // decrypt secret and modify
            *v = crypto::decrypt(dec, key).unwrap()
        }
    });
    map.insert("data".to_owned(), data);

    // return to JS
    to_value(&StringMap(map)).unwrap()
}

/// Encrypt secrets and serialize map
#[wasm_bindgen]
pub fn serialize_storage(storage: JsValue, sort: &str, key: &str) -> String {
    // deserialize from JS
    let storage: StringMap = from_value(storage).unwrap();

    // new map and iterate through storage entries
    let mut map = HashMap::new();
    for (k, v) in storage.0 {
        // hash secret name
        let name = crypto::hash_name(&k);

        // encrypt secret and name
        let enc_secret = crypto::encrypt(&v, key).unwrap();
        let enc_name = crypto::encrypt(&k, key).unwrap();

        // hex encode secret and name
        let hex_secret = crypto::hex_encode(enc_secret);
        let hex_name = crypto::hex_encode(enc_name);

        // add secret and secret name
        map.insert(format!("{name}_secret"), hex_secret);
        map.insert(format!("{name}_secret_name"), hex_name);
    }

    // encrypt sort
    let enc_sort = crypto::encrypt(sort, key).unwrap();
    let hex_sort = crypto::hex_encode(enc_sort);
    map.insert("secrets_sort".to_owned(), hex_sort);

    // return serialized
    serialize(&map)
}
