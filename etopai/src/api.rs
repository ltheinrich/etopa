//! API handlers

use crate::{jsonify, SharedData};
use etopa::Fail;
use json::{parse, JsonValue};
use lhi::server::{HttpMethod, HttpRequest};
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

/// Register handler
pub fn register(req: HttpRequest, shared: Arc<RwLock<SharedData>>) -> Result<Vec<u8>, Fail> {
    let val = default_check(&req)?;
    let data = to_map(&val);
    let name = get(&data, "username")?;
    let password_hash = get(&data, "password_hash")?;
    // let user_data = &mut shared.write().unwrap().user_data;
    Ok(jsonify(object!(hello: "world")))
}

/// Get value or fail
fn get<'a>(data: &'a BTreeMap<&str, &JsonValue>, key: &str) -> Result<&'a JsonValue, Fail> {
    Ok(*data
        .get(key)
        .ok_or_else(|| Fail::new(format!("{} required", key)))?)
}

/// Convert JsonValue to map
fn to_map<'a>(val: &'a JsonValue) -> BTreeMap<&'a str, &'a JsonValue> {
    let mut data = BTreeMap::new();
    val.entries().for_each(|(k, v)| {
        data.insert(k, v);
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
