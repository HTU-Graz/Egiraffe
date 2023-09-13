pub mod v1;

use axum::{response::IntoResponse, routing::get, Json};
use serde_json::json;

use crate::Router;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(api_greeting))
        .nest("/v1", v1::routes())
}

async fn api_greeting() -> impl IntoResponse {
    Json(json!({
        "message": "Welcome to the Egiraffe API!",
        "backend_version": env!("CARGO_PKG_VERSION"),
    }))
}
