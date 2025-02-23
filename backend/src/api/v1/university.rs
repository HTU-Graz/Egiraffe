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

use crate::{
    api::api_greeting,
    data::{Course, OwnedUniversity, RgbColor},
    db::{self, DB_POOL},
};

pub fn routes() -> Router {
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
    pub email_domain_names: Vec<String>,
    pub homepage_url: String,
    pub cms_url: String,
    pub background_color: RgbColor,
    pub text_color: RgbColor,
}

async fn handle_create_university(
    Json(university): Json<CreateUniversityReq>,
) -> impl IntoResponse {
    let mut tx = (*DB_POOL.get().unwrap()).begin().await.unwrap();

    let university = OwnedUniversity {
        id: Uuid::nil(), // This will be set by the database
        full_name: university.full_name,
        mid_name: university.mid_name,
        short_name: university.short_name,
        email_domain_names: university.email_domain_names,
        homepage_url: university.homepage_url,
        cms_url: university.cms_url,
        background_color: university.background_color,
        text_color: university.text_color,
    };

    let db_action_result = db::university::create_university(&mut tx, university).await;

    if let Err(error) = db_action_result {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": error.to_string(),
            })),
        );
    }

    let id = db_action_result.unwrap();
    tx.commit().await.unwrap();

    (StatusCode::OK, Json(json!({ "success": true, "id": id })))
}

async fn handle_replace_university(Json(course): Json<Course>) -> impl IntoResponse {
    let mut tx = (*DB_POOL.get().unwrap()).begin().await.unwrap();

    let db_action_result = db::course::replace_course(&mut tx, course).await;

    if let Err(error) = db_action_result {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": error.to_string(),
            })),
        );
    }

    tx.commit().await.unwrap();

    (StatusCode::OK, Json(json!({ "success": true })))
}
