//! API handlers

use crate::{jsonify, SharedData};
use etopa::{argon2_hash, argon2_verify, random, Fail};
use json::{parse, JsonValue};
use lhi::server::{HttpMethod, HttpRequest};
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

/// Delete account handler
pub fn delete(req: HttpRequest, shared: Arc<RwLock<SharedData>>) -> Result<Vec<u8>, Fail> {
    // parse
    let val = default_check(&req)?;
    let data = to_map(&val);

    // get values
    let username = get_str(&data, "username")?;
    let password = get_str(&data, "password")?;

    // get shared
    let user_data = &mut shared.write().unwrap().user_data;
    let mut users = user_data.parse()?;

    // get password hash from db
    match users.get(username) {
        Some(password_hash) => {
            // verify argon2 password hash
            let password_verified = argon2_verify(password_hash, password.as_bytes());
            if !password_verified {
                return Fail::from("unauthenticated");
            }

            // delete user and return success
            users.remove(username);
            user_data.serialize(&users)?;
            Ok(jsonify(object!(success: true)))
        }
        None => Fail::from("username does not exist"),
    }
}

/// Login handler
pub fn login(req: HttpRequest, shared: Arc<RwLock<SharedData>>) -> Result<Vec<u8>, Fail> {
    // parse
    let val = default_check(&req)?;
    let data = to_map(&val);

    // get values
    let username = get_str(&data, "username")?;
    let password = get_str(&data, "password")?;

    // get shared
    let user_data = &mut shared.write().unwrap().user_data;
    let users = user_data.parse()?;

    // get password hash from db
    match users.get(username) {
        Some(password_hash) => {
            // verify argon2 password hash
            let password_verified = argon2_verify(password_hash, password.as_bytes());

            // return success
            Ok(jsonify(object!(success: password_verified)))
        }
        None => Fail::from("username does not exist"),
    }
}

/// Register handler
pub fn register(req: HttpRequest, shared: Arc<RwLock<SharedData>>) -> Result<Vec<u8>, Fail> {
    // parse
    let val = default_check(&req)?;
    let data = to_map(&val);

    // get values
    let username = get_str(&data, "username")?;
    let password = get_str(&data, "password")?;

    // get shared
    let user_data = &mut shared.write().unwrap().user_data;
    let mut users = user_data.parse()?;

    // check if user already exists
    if users.contains_key(username) {
        return Fail::from("username already exists");
    }

    // argon2 hash password
    let salt = random(10);
    let password_argon2 = argon2_hash(password.as_bytes(), &salt)?;

    // modify users
    users.insert(username.to_string(), password_argon2);
    user_data.serialize(&users)?;

    // return success
    Ok(jsonify(object!(success: true)))
}

/// Get value as string or fail
fn get_str<'a>(data: &'a BTreeMap<String, &JsonValue>, key: &str) -> Result<&'a str, Fail> {
    get(data, key)?
        .as_str()
        .ok_or_else(|| Fail::new(format!("{} is not a string", key)))
}

/// Get value or fail
fn get<'a>(data: &'a BTreeMap<String, &JsonValue>, key: &str) -> Result<&'a JsonValue, Fail> {
    Ok(*data
        .get(key)
        .ok_or_else(|| Fail::new(format!("{} required", key)))?)
}

/// Convert JsonValue to map
fn to_map<'a>(val: &'a JsonValue) -> BTreeMap<String, &'a JsonValue> {
    let mut data = BTreeMap::new();
    val.entries().for_each(|(k, v)| {
        data.insert(k.to_lowercase(), v);
    });
    data
}

/// Perform default checks and return JsonValue
fn default_check(req: &HttpRequest) -> Result<JsonValue, Fail> {
    is_post(req)?;
    has_body(req)?;
    is_json(&req)
}

/// Check if request body is JSON and return if is
fn is_json(req: &HttpRequest) -> Result<JsonValue, Fail> {
    // check content-type
    if req.headers().get("content-type").unwrap_or(&"") != &"application/json" {
        // wrong content-type
        return Fail::from("Content-Type is not application/json");
    }

    // parse json
    parse(req.body()).or_else(|_| Fail::from("Request body is not JSON"))
}

/// Check if request method is POST
fn is_post(req: &HttpRequest) -> Result<(), Fail> {
    // check method
    match req.method() {
        // is POST
        HttpMethod::POST => Ok(()),
        // not POST
        _ => Fail::from("POST method required"),
    }
}

/// Check if request has body
fn has_body(req: &HttpRequest) -> Result<(), Fail> {
    // check if is empty (trim whitespace first)
    if req.body().trim().is_empty() {
        // empty body
        Fail::from("Empty body")
    } else {
        // has body
        Ok(())
    }
}
