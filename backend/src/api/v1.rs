use axum::routing::get;

use crate::Router;

use super::api_greeting;

pub fn routes() -> Router {
    Router::new().route("/", get(api_greeting).post(api_greeting).put(api_greeting))
}
