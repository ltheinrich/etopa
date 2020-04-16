//! API utils

use etopa::Fail;
use json::JsonValue;
use lhi::server::{respond, ResponseData};
use std::collections::BTreeMap;
use std::fmt::Display;
use std::str::FromStr;

/// Get value as string or fail
pub fn get_str<'a>(data: &BTreeMap<String, &'a str>, key: &str) -> Result<&'a str, Fail> {
    Ok(*data
        .get(key)
        .ok_or_else(|| Fail::new(format!("{} required", key)))?)
}

/// Get value or fail
pub fn get<T: FromStr>(data: &BTreeMap<String, &str>, key: &str) -> Result<T, Fail> {
    get_str(data, key)?
        .parse()
        .or_else(|_| Fail::from(format!("{} is not correct type", key)))
}

/// Convert JsonValue to response
pub fn jsonify(value: JsonValue) -> Vec<u8> {
    respond(value.to_string(), "application/json", cors_headers())
}

/// Convert error message into json format error
pub fn json_error<E: Display>(err: E) -> Vec<u8> {
    jsonify(object!(error: format!("{}", err)))
}

pub fn cors_headers() -> Option<ResponseData<'static>> {
    let mut resp_data = ResponseData::new();
    resp_data.headers.insert("access-control-allow-origin", "*");
    resp_data
        .headers
        .insert("access-control-allow-headers", "*");
    resp_data
        .headers
        .insert("access-control-allow-methods", "*");
    Some(resp_data)
}
