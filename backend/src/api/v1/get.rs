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

use crate::{api::api_greeting, db, AppState};

pub fn routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(api_greeting).post(api_greeting).put(api_greeting))
        .route("/courses", put(handle_get_courses))
        .route("/uploads", put(handle_get_uploads))
        .route("/universities", put(handle_get_universities))
        .route("/me", put(handle_get_me))
}

#[derive(Debug, Deserialize)]
pub struct GetUploadsReq {
    pub course_id: Uuid,
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

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "courses": db_action_result.unwrap(),
        })),
    )
}

async fn handle_get_uploads(
    State(db_pool): State<AppState>,
    Json(course): Json<GetUploadsReq>,
) -> impl IntoResponse {
    log::info!("Get uploads for course {}", course.course_id);

    let maybe_uploads = db::upload::get_uploads_of_course(&db_pool, course.course_id).await;

    let Ok(uploads) = maybe_uploads else {
        log::error!("Failed to get courses: {}", maybe_uploads.unwrap_err());

        // TODO return a more specific error message (e.g. 404 if course doesn't exist)
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": "Failed to get courses",
            })),
        );
    };

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "uploads": uploads,
        })),
    )
}

async fn handle_get_universities(State(db_pool): State<AppState>) -> impl IntoResponse {
    let maybe_universities = db::university::get_universities(&db_pool).await;

    let Ok(universities) = maybe_universities else {
        log::error!(
            "Failed to get universities: {}",
            maybe_universities.unwrap_err()
        );

        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": "Failed to get universities",
            })),
        );
    };

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "universities": universities,
        })),
    )
}

async fn handle_get_me(State(db_pool): State<AppState>) -> impl IntoResponse {
    // TODO implement this
    return (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({
            "success": false,
            "message": "This is not implemented yet",
        })),
    );
}
