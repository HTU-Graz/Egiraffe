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
    data::Course,
    db::{self, DB_POOL},
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(api_greeting).post(api_greeting).put(api_greeting))
        .route("/create", put(handle_create_course))
        .route("/replace", put(handle_replace_course))
}

#[derive(Debug, Deserialize)]
pub struct CreateCourseReq {
    pub name: String,

    /// The ID of the university this course belongs to
    pub held_at: Uuid,
}

async fn handle_create_course(Json(course): Json<CreateCourseReq>) -> impl IntoResponse {
    let db_pool = *DB_POOL.get().unwrap();

    let course = Course {
        id: Uuid::new_v4(),
        name: course.name,
        held_at: course.held_at,
    };

    let db_action_result = db::course::create_course(&db_pool, &course).await;

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
        Json(json!({ "success": true, "course": course })),
    )
}

async fn handle_replace_course(Json(course): Json<Course>) -> impl IntoResponse {
    let db_pool = *DB_POOL.get().unwrap();

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
