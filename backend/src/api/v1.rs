use axum::routing::get;

use crate::Router;

use super::placeholder_api;

pub fn routes() -> Router {
    Router::new().route(
        "/",
        get(placeholder_api)
            .post(placeholder_api)
            .put(placeholder_api),
    )
}
