use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::{api::api_greeting, data::Prof, db, AppState};

pub fn routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(api_greeting).post(api_greeting).put(api_greeting))
        .route("/create", put(handle_create_prof))
        .route("/replace", put(handle_replace_prof))
}

#[derive(Debug, Deserialize)]
pub struct CreateProfReq {
    pub name: String,
}

async fn handle_create_prof(
    State(db_pool): State<AppState>,
    Json(prof): Json<CreateProfReq>,
) -> impl IntoResponse {
    let prof = Prof {
        id: Uuid::new_v4(),
        name: prof.name,
    };

    let db_action_result = db::prof::create_prof(&db_pool, &prof).await;

    if let Err(error) = db_action_result {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": error.to_string(),
            })),
        );
    }

    (
        StatusCode::OK,
        Json(json!({ "success": true, "prof": prof })),
    )
}

async fn handle_replace_prof(
    State(db_pool): State<AppState>,
    Json(prof): Json<Prof>,
) -> impl IntoResponse {
    let db_action_result = db::prof::update_prof(&db_pool, &prof).await;

    if let Err(error) = db_action_result {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": error.to_string(),
            })),
        );
    }

    (StatusCode::OK, Json(json!({ "success": true })))
}
