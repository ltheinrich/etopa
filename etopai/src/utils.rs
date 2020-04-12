//! API utils

use etopa::Fail;
use json::{parse, JsonValue};
use lhi::server::{HttpMethod, HttpRequest};
use std::collections::BTreeMap;

/// Get value as string or fail
pub fn get_str<'a>(data: &'a BTreeMap<String, &JsonValue>, key: &str) -> Result<&'a str, Fail> {
    get(data, key)?
        .as_str()
        .ok_or_else(|| Fail::new(format!("{} is not a string", key)))
}

/// Get value or fail
pub fn get<'a>(data: &'a BTreeMap<String, &JsonValue>, key: &str) -> Result<&'a JsonValue, Fail> {
    Ok(*data
        .get(key)
        .ok_or_else(|| Fail::new(format!("{} required", key)))?)
}

/// Convert JsonValue to map
pub fn to_map<'a>(val: &'a JsonValue) -> BTreeMap<String, &'a JsonValue> {
    let mut data = BTreeMap::new();
    val.entries().for_each(|(k, v)| {
        data.insert(k.to_lowercase(), v);
    });
    data
}

/// Perform default checks and return JsonValue
pub fn default_check(req: &HttpRequest) -> Result<JsonValue, Fail> {
    is_post(req)?;
    has_body(req)?;
    is_json(&req)
}

/// Check if request body is JSON and return if is
pub fn is_json(req: &HttpRequest) -> Result<JsonValue, Fail> {
    // check content-type
    if req.headers().get("content-type").unwrap_or(&"") != &"application/json" {
        // wrong content-type
        return Fail::from("content-type is not application/json");
    }

    // parse json
    parse(req.body()).or_else(|_| Fail::from("request body is not JSON"))
}

/// Check if request method is POST
pub fn is_post(req: &HttpRequest) -> Result<(), Fail> {
    // check method
    match req.method() {
        // is POST
        HttpMethod::POST => Ok(()),
        // not POST
        _ => Fail::from("POST method required"),
    }
}

/// Check if request has body
pub fn has_body(req: &HttpRequest) -> Result<(), Fail> {
    // check if is empty (trim whitespace first)
    if req.body().trim().is_empty() {
        // empty body
        Fail::from("empty body")
    } else {
        // has body
        Ok(())
    }
}
