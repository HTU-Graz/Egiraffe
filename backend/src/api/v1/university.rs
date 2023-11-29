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

use crate::{api::api_greeting, data::OwnedUniversity, db, AppState};

pub fn routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(api_greeting).post(api_greeting).put(api_greeting))
        .route("/create", put(handle_create_university))
        .route("/replace", put(handle_replace_university))
}

#[derive(Debug, Deserialize)]
pub struct CreateUniversityReq {
    pub full_name: String,
    pub mid_name: String,
    pub short_name: String,
    pub domain_names: Vec<String>,
}

async fn handle_create_university(
    State(db_pool): State<AppState>,
    Json(university): Json<CreateUniversityReq>,
) -> impl IntoResponse {
    let university = OwnedUniversity {
        id: Uuid::new_v4(),
        full_name: university.full_name,
        mid_name: university.mid_name,
        short_name: university.short_name,
        domain_names: university.domain_names,
    };

    let db_action_result = db::university::create_university(&db_pool, university).await;

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

async fn handle_replace_university(
    State(db_pool): State<AppState>,
    Json(course): Json<Course>,
) -> impl IntoResponse {
    let db_action_result = db::course::replace_course(&db_pool, course).await;

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
