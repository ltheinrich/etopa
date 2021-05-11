//! Etopa for Web

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[macro_use]
extern crate serde_derive;

#[cfg(target_arch = "wasm32")]
use etopa::wasm_bindgen::{self, prelude::*};
use etopa::{
    crypto,
    data::{parse, serialize},
    totp::Generator,
};
use std::collections::BTreeMap;

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
        Ok(gen) => gen
            .token_at(time_millis / 1000)
            .unwrap_or_else(|_| "invalid secret".to_string()),
        _ => "invalid secret".to_string(),
    }
}

#[derive(Serialize, Deserialize)]
struct StringMap(BTreeMap<String, String>);

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
            let dec = crypto::hex_decode(&v).unwrap();

            // decrypt secret and modify
            *v = crypto::decrypt(dec, key).unwrap()
        }
    });
    map.insert("data".to_owned(), data);

    // return to JS
    JsValue::from_serde(&StringMap(map)).unwrap()
}

/// Encrypt secrets and serialize map
#[wasm_bindgen]
pub fn serialize_storage(storage: JsValue, key: &str) -> String {
    // deserialize from JS
    let storage: StringMap = storage.into_serde().unwrap();

    // new map and iterate through storage entries
    let mut map = BTreeMap::new();
    let mut sort = String::with_capacity(storage.0.len() * 65);
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
        map.insert(format!("{}_secret", name), hex_secret);
        map.insert(format!("{}_secret_name", name), hex_name);
        sort.push_str(&name);
        sort.push(',');
    }

    // encrypt sort
    let enc_sort = crypto::encrypt(&sort, key).unwrap();
    let hex_sort = crypto::hex_encode(enc_sort);
    map.insert("secrets_sort".to_owned(), hex_sort);

    // return serialized
    serialize(&map)
}

/// Encrypt secrets sort
#[wasm_bindgen]
pub fn encrypt_sort(storage: JsValue, key: &str) -> String {
    // deserialize from JS
    let storage: StringMap = storage.into_serde().unwrap();

    // plaintext sort string
    let mut sort = String::with_capacity(storage.0.len() * 65);
    for (k, _) in storage.0 {
        // hash secret name
        let name = crypto::hash_name(&k);
        sort.push_str(&name);
        sort.push(',');
    }

    // encrypt sort and return
    let enc_sort = crypto::encrypt(&sort, key).unwrap();
    crypto::hex_encode(enc_sort)
}
