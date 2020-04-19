//! Etopa for Web
#![cfg(target_arch = "wasm32")]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use etopa::{
    crypto,
    data::{decrypt, encrypt},
    totp::Generator,
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
pub fn decrypt_storage(data: &[u8], key: &str) -> String {
    let data = if data.len() > 1 {
        &data[..(data.len() - 2)]
    } else {
        data
    };
    decrypt(data, key).unwrap()
}

#[wasm_bindgen]
pub fn encrypt_storage(data: &str, key: &str) -> Vec<u8> {
    encrypt(data, key).unwrap()
}

#[wasm_bindgen]
pub fn gen_token(secret: &str, time_millis: u64) -> String {
    Generator::new(secret)
        .unwrap()
        .token_at(time_millis / 1000)
        .unwrap()
}
