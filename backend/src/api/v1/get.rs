use std::{path::PathBuf, vec};

use anyhow::Context;
use axum::{
    body::Body,
    extract::State,
    http::{header, StatusCode},
    response::{AppendHeaders, IntoResponse},
    routing::{get, put},
    Extension, Json, Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use serde::Deserialize;
use serde_json::json;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

use crate::{
    api::{api_greeting, v1::auth::make_dead_cookie},
    data::{File, RedactedUser, Upload},
    db::{self, DB_POOL},
};

use super::SESSION_COOKIE_NAME;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(api_greeting).post(api_greeting).put(api_greeting))
        .route("/courses", put(handle_get_courses))
        .route("/uploads", put(handle_get_uploads))
        .route("/universities", put(handle_get_universities))
        .route("/me", put(handle_get_me))
        .route("/file", put(handle_get_file))
        .route("/files-of-upload", put(handle_get_files_of_upload))
        .route("/prof", put(handle_get_prof))
        .route("/my-ecs-balance", put(handle_get_my_ecs))
}

/// Handles requests to get the user's own current ECs balance
async fn handle_get_my_ecs(
    Extension(current_user_id): Extension<Uuid>, // Get the user ID from the session
) -> impl IntoResponse {
    let db_pool = *DB_POOL.get().unwrap();

    // Most `/get` endpoints do not require authentication; this one does
    if current_user_id.is_nil() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "success": false, "message": "Unauthorized" })),
        );
    }

    // let maybe_ecs = sqlx::query_file_as!(i32, "src/db/sql/get_available_ecs.sql", current_user_id)
    //     .fetch_one(&*db_pool)
    //     .await;
    let maybe_ecs: anyhow::Result<(f64,)> =
        sqlx::query_as(include_str!("../../db/sql/get_available_ecs.sql"))
            .bind(&current_user_id)
            .fetch_one(&*db_pool)
            .await
            .context("Failed to get ECs");

    let Ok((ecs,)) = maybe_ecs else {
        log::error!("Failed to get ECs: {:#?}", maybe_ecs.unwrap_err());

        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": "Failed to get ECs",
            })),
        );
    };

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "ecs_balance": ecs,
        })),
    )
}

#[derive(Debug, Deserialize)]
pub struct GetUploadsReq {
    pub course_id: Uuid,
    pub sorting: Option<db::upload::Sorting>,
}

async fn handle_get_courses() -> impl IntoResponse {
    let db_pool = *DB_POOL.get().unwrap();

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

    // TODO add some kind of upload approval status

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "courses": courses,
        })),
    )
}

async fn handle_get_uploads(Json(course): Json<GetUploadsReq>) -> impl IntoResponse {
    let db_pool = *DB_POOL.get().unwrap();

    log::info!("Get uploads for course {}", course.course_id);

    let maybe_uploads =
        db::upload::get_uploads_of_course(&db_pool, course.course_id, course.sorting).await;

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

async fn handle_get_universities() -> impl IntoResponse {
    let db_pool = *DB_POOL.get().unwrap();

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

async fn handle_get_me(cookie_jar: CookieJar) -> impl IntoResponse {
    let db_pool = *DB_POOL.get().unwrap();

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
        return (
            StatusCode::UNAUTHORIZED,
            cookie_jar,
            Json(generic_error_response),
        );
    };

    // Get the user from the database
    let maybe_user = db::user::get_user_by_session(&db_pool, session_cookie.value()).await;
    let Ok(user) = maybe_user else {
        log::error!(
            "Failed to get user, removing session cookie: {}",
            maybe_user.unwrap_err()
        );

        return (
            StatusCode::UNAUTHORIZED,
            cookie_jar.add(make_dead_cookie()),
            Json(generic_error_response),
        );
    };

    return (
        StatusCode::OK,
        cookie_jar,
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

/// Handles the actual download of a file to a client
async fn handle_get_file(
    Extension(current_user_id): Extension<Uuid>, // Get the user ID from the session
    Json(req): Json<GetFileReq>,
) -> impl IntoResponse {
    let db_pool = *DB_POOL.get().unwrap();

    // Most `/get` endpoints do not require authentication; this one does
    if current_user_id.is_nil() {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({ "success": false, "message": "Unauthorized" })),
        ));
    }

    let maybe_file = db::file::get_file(&db_pool, req.file_id).await;

    let Ok(file) = maybe_file else {
        log::error!("Failed to get file: {}", maybe_file.unwrap_err());

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": "Failed to get file",
            })),
        ));
    };

    // Prepare the download logic
    let do_download_to_user = async {
        let fs_file =
            tokio::fs::File::open(PathBuf::from("uploads").join(file.id.to_string())).await;

        let fs_file = fs_file.unwrap(); // TODO handle error

        let stream = ReaderStream::new(fs_file);
        let body = Body::from_stream(stream);

        return Ok((
            StatusCode::OK,
            [
                (header::CONTENT_TYPE, file.mime_type),
                (
                    header::CONTENT_DISPOSITION,
                    format!("attachment; filename={}", file.name),
                ),
            ],
            body,
        ));
    };

    let Ok(upload) = db::file::get_upload_of_file(&db_pool, file.id).await else {
        log::error!("Failed to get upload of file: {}", file.id);

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": "Failed to get upload",
            })),
        ));
    };

    // A user is always authorized to access their own files
    if upload.uploader == current_user_id {
        log::info!(
            "User {current_user_id} is authorized (owner) to access file {}",
            file.id
        );
        return do_download_to_user.await;
    } else {
        log::info!("User {current_user_id} does not own file {}", file.id);
        // Option 1: the file is owned by the user
        // or
        // Option 2: the file has been approved by a moderator and by the uploader
        if !(file.approval_mod && file.approval_uploader) {
            log::info!(
                "User {current_user_id} is not authorized to access file {}",
                file.id
            );

            // The user has not purchased this file
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "success": false,
                    "message": "This file lacks approval from a moderator and/or the uploader",
                })),
            ));
        }
    }

    // Check if there is a valid purchase for this file
    let maybe_purchase = db::purchase::get_purchase(&db_pool, current_user_id, file.id).await;
    let Ok(purchase) = maybe_purchase else {
        log::error!("Failed to get purchase: {}", maybe_purchase.unwrap_err());

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": "Failed to get purchase",
            })),
        ));
    };

    if purchase.is_none() {
        log::info!(
            "User {current_user_id} is not authorized to access file {}",
            file.id
        );

        // The user has not purchased this file
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "success": false,
                "message": "No valid purchase for this file and user",
            })),
        ));
    };

    log::info!(
        "User {current_user_id} is authorized (purchase) to access file {}",
        file.id
    );

    do_download_to_user.await
}

#[derive(Debug, Deserialize)]
pub struct GetUploadReq {
    pub upload_id: Uuid,
}

async fn handle_get_files_of_upload(
    Extension(current_user_id): Extension<Uuid>, // Get the user ID from the session
    Json(upload): Json<GetUploadReq>,
) -> impl IntoResponse {
    let db_pool = *DB_POOL.get().unwrap();

    log::info!("Get details for upload {}", upload.upload_id);

    let maybe_files = db::file::get_files_and_join_upload(&db_pool, upload.upload_id).await;

    let Ok(files) = maybe_files else {
        log::error!("Failed to get files: {}", maybe_files.unwrap_err());

        // TODO return a more specific error message (e.g. 404 if course doesn't exist)
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": "Failed to get files & upload info",
            })),
        );
    };

    let original_files_count = files.len();

    // Option 1: the file is owned by the user
    // or
    // Option 2: the file has been approved by a moderator and by the uploader
    let show_file_predicate = |(file, upload): &(File, Upload)| {
        (upload.uploader == current_user_id) || (file.approval_mod && file.approval_uploader)
    };

    // Filter out files that have not been approved
    // FIXME let people get their own files even if they are not approved
    let files = files
        .into_iter()
        .filter(show_file_predicate)
        .map(|(file, _)| file)
        .collect::<Vec<_>>();

    // Get the upload info
    let maybe_upload = get_upload(&db_pool, upload.upload_id).await;

    let Ok(upload) = maybe_upload else {
        log::error!("Failed to get upload: {}", maybe_upload.unwrap_err());

        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": "Failed to get upload",
            })),
        );
    };

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "files": files,
            "total_files_count": original_files_count,
            "upload": upload,
        })),
    )
}

#[derive(Debug, Deserialize)]
pub struct GetProfReq {
    pub prof_id: Uuid,
}

async fn handle_get_prof(
    Extension(current_user_id): Extension<Uuid>, // Get the user ID from the session
    Json(prof_req): Json<GetProfReq>,
) -> impl IntoResponse {
    let db_pool = *DB_POOL.get().unwrap();

    log::info!("Get details for prof {}", prof_req.prof_id);

    if current_user_id.is_nil() {
        log::info!("User is not logged in; resolving profs requires authentication");

        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "success": false,
                "message": "Resolving profs requires authentication",
            })),
        );
    }

    let maybe_prof = db::prof::get_prof(&db_pool, prof_req.prof_id).await;

    let Ok(prof) = maybe_prof else {
        log::error!("Failed to get prof: {}", maybe_prof.unwrap_err());

        // TODO return a more specific error message (e.g. 404 if prof doesn't exist)
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": "Failed to get prof",
            })),
        );
    };

    let Some(prof) = prof else {
        log::info!("Prof {} does not exist", prof_req.prof_id);

        // TODO return a more specific error message (e.g. 404 if prof doesn't exist)
        return (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "message": "Prof does not exist",
            })),
        );
    };

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "prof": prof,
        })),
    )
}

async fn get_upload(db_pool: &sqlx::PgPool, upload_id: Uuid) -> anyhow::Result<Upload> {
    let upload = sqlx::query_as!(
        Upload,
        "
        SELECT
            upload.id,
            upload_name AS name,
            description,
            price,
            uploader,
            upload_date,
            last_modified_date,
            belongs_to,
            held_by
        FROM
            upload
        WHERE
            upload.id = $1
        ",
        upload_id,
    )
    .fetch_one(db_pool)
    .await
    .context("Failed to get upload")?;

    Ok(upload)
}
