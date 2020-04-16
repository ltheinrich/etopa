//! Etopa for Web
#![cfg(target_arch = "wasm32")]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use etopa::{
    crypto,
    data::{DataMap, SecureStorage},
    wasm_bindgen::{self, prelude::*},
};

#[wasm_bindgen]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn hash_password(password: &str, username: &str) -> String {
    crypto::hash_password(password.as_bytes(), username.as_bytes())
}

#[wasm_bindgen]
pub fn decrypt_storage(data: &[u8], key: &str) -> JsValue {
    if data.len() > 10 {
        if let Ok(storage) = SecureStorage::new(&data[..(data.len() - 2)], key) {
            return JsValue::from_serde(storage.data()).unwrap();
        }
    }
    JsValue::NULL
}

#[wasm_bindgen]
pub fn encrypt_storage(data: &JsValue, key: &str) -> Vec<u8> {
    let map: DataMap = data.into_serde().unwrap();
    let storage = SecureStorage::from_map(map, key);
    storage.encrypt().unwrap()
}
