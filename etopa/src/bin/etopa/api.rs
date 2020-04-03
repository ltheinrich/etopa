//! API handlers

use crate::{json_error, jsonify};
use lhi::server::HttpRequest;

/// Test handler
pub fn test(req: HttpRequest) -> Vec<u8> {
    if req.body().is_empty() {
        return json_error("Empty body");
    }
    jsonify(object!(hello: "world"))
}
