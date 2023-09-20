use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
    Json, Router,
};
use serde::Serialize;
use serde_json::json;
use uuid::Uuid;

use crate::{api::api_greeting, data::Course, db, AppState};

pub fn routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(api_greeting).post(api_greeting).put(api_greeting))
        .route("/create", put(handle_create_course))
}

#[derive(Debug, Serialize)]
pub struct CreateCourseReq {
    pub name: String,

    /// The ID of the university this course belongs to
    pub held_at: Uuid,
}

async fn handle_create_course(
    State(db_pool): State<AppState>,
    Json(course): Json<CreateCourseReq>,
) -> impl IntoResponse {
    let course = Course {
        id: Uuid::new_v4(),
        name: course.name,
        held_at: course.held_at,
    };

    let course_creation_status = db::course::create_course(&db_pool, course).await;

    if course_creation_status.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": course_creation_status.unwrap_err().to_string(),
            })),
        );
    }

    (StatusCode::OK, Json(json!({ "success": true })))
}
