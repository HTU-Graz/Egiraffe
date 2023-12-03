use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
    Extension, Json, Router,
};
use axum_extra::extract::CookieJar;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::{api::api_greeting, data::RedactedUser, db, AppState};

use super::SESSION_COOKIE_NAME;

pub fn routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(api_greeting).post(api_greeting).put(api_greeting))
        .route("/courses", put(handle_get_courses))
        .route("/uploads", put(handle_get_uploads))
        .route("/universities", put(handle_get_universities))
        .route("/me", put(handle_get_me))
        .route("/file", put(handle_get_file))
}

#[derive(Debug, Deserialize)]
pub struct GetUploadsReq {
    pub course_id: Uuid,
}

async fn handle_get_courses(State(db_pool): State<AppState>) -> impl IntoResponse {
    let maybe_courses = db::course::get_courses(&db_pool).await;

    let Ok(courses) = maybe_courses else {
        log::error!("Failed to get courses: {}", maybe_courses.unwrap_err());

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
            "courses": courses,
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

async fn handle_get_me(
    State(db_pool): State<AppState>,
    cookie_jar: CookieJar,
) -> impl IntoResponse {
    // We return a generic error response if the user is not logged in
    //  to avoid leaking private information
    let generic_error_response = json!({
        "success": false,
        "message": "You are not logged in",
    });

    // TODO we should probably have a middleware for this

    // Get the session cookie from the cookie jar
    let Some(session_cookie) = cookie_jar.get(SESSION_COOKIE_NAME) else {
        log::info!("No session cookie");
        return (StatusCode::UNAUTHORIZED, Json(generic_error_response));
    };

    // Get the user from the database
    let maybe_user = db::user::get_user_by_session(&db_pool, session_cookie.value()).await;
    let Ok(user) = maybe_user else {
        log::error!("Failed to get user: {}", maybe_user.unwrap_err());
        return (StatusCode::UNAUTHORIZED, Json(generic_error_response));
    };

    return (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "user": RedactedUser::from(user), // hide sensitive information
        })),
    );
}

#[derive(Debug, Deserialize)]
pub struct GetFileReq {
    pub file_id: Uuid,
}

async fn handle_get_file(
    State(db_pool): State<AppState>,
    Extension(current_user_id): Extension<Uuid>, // Get the user ID from the session
    Json(req): Json<GetFileReq>,
) -> impl IntoResponse {
    // Most `/get` endpoints do not require authentication; this one does
    if current_user_id.is_nil() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Unauthorized" })),
        );
    }

    let maybe_file = db::file::get_file(&db_pool, req.file_id).await;

    let Ok(file) = maybe_file else {
        log::error!("Failed to get file: {}", maybe_file.unwrap_err());

        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": "Failed to get file",
            })),
        );
    };

    // A user is always authorized to access their own files
    if file.upload_id == current_user_id {
        // FIXME actually send the file
        return (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "file": file,
            })),
        );
    }

    // Check if there is a valid purchase for this file
    let maybe_purchase = db::purchase::get_purchase(&db_pool, current_user_id, file.id).await;
    let Ok(purchase) = maybe_purchase else {
        log::error!("Failed to get purchase: {}", maybe_purchase.unwrap_err());

        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": "Failed to get purchase",
            })),
        );
    };

    let Some(purchase) = purchase else {
        // The user has not purchased this file
        return (
            StatusCode::OK,
            Json(json!({
                "success": false,
                "message": "No valid purchase for this file and user",
            })),
        );
    };

    // FIXME actually send the file
    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "file": file,
        })),
    )
}
