pub mod v1;

use axum::{response::IntoResponse, routing::get};

use crate::Router;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(placeholder_api))
        .nest("/v1", v1::routes())
}

async fn placeholder_api() -> impl IntoResponse {
    "Egiraffe API goes here (todo)"
}
