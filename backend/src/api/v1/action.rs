use axum::{routing::get, Router};

use crate::{api::api_greeting, AppState};

// Handles resource-modifying requests from authenticated users
pub fn routes(state: &AppState) -> Router<AppState> {
    Router::new().route("/", get(api_greeting).post(api_greeting).put(api_greeting))
    // .route("/courses", put(handle_get_courses))
    // .route("/uploads", put(handle_get_uploads))
    // .route("/universities", put(handle_get_universities))
    // .route("/me", put(handle_get_me))
}
