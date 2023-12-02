use axum::{http::StatusCode, Json};
use serde_json::json;

#[inline]
pub fn bad_request(message: &str) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::BAD_REQUEST,
        Json(json!({
            "success": false,
            "message": message,
        })),
    )
}
