// TODO remove this allow and implement the API
#![allow(dead_code, unused_imports)]

use axum::{response::IntoResponse, routing::get, Router};

pub fn routes() {
    // Router::new().nest("/v1", v1::routes())
}

pub mod v1 {

    use super::*;

    pub fn routes() {}
}

async fn placeholder_api() -> impl IntoResponse {
    "Egiraffe API goes here (todo)"
}
