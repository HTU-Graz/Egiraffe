use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
    Json, Router,
};
use serde_json::json;

use crate::{api::api_greeting, db, AppState};

pub fn routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(api_greeting).post(api_greeting).put(api_greeting))
        .route("/courses", put(handle_get_courses))
}

async fn handle_get_courses(State(db_pool): State<AppState>) -> impl IntoResponse {
    let db_action_result = db::course::get_courses(&db_pool).await;

    if let Err(error) = db_action_result {
        log::error!("Failed to get courses: {}", error);

        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": "Failed to get courses",
            })),
        );
    }

    (StatusCode::OK, Json(json!({ "success": true })))
}
