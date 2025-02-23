//! Content moderation API endpoints

use std::path::PathBuf;

use anyhow::Context;
use axum::{
    body::Body,
    extract::State,
    http::{header, StatusCode},
    response::IntoResponse,
    routing::{get, put},
    Extension, Json, Router,
};
use chrono::NaiveDateTime;
use serde::Deserialize;
use serde_json::json;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

use crate::{
    api::api_greeting,
    data::File,
    db::{self, DB_POOL},
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(api_greeting).post(api_greeting).put(api_greeting))
        .route("/modify-upload", put(handle_modify_upload))
        .route("/modify-file", put(handle_modify_file))
        .route("/get-all-uploads", put(handle_get_all_uploads))
        .route("/get-all-files", put(handle_get_all_files))
        .route("/download-file-as-mod", put(download_file_as_mod))
}

#[derive(Debug, Deserialize)]
pub struct ModifyUploadRequest {
    pub id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<i16>,
    pub uploader: Option<Uuid>,
    pub upload_date: Option<NaiveDateTime>,
    pub last_modified_date: Option<NaiveDateTime>,

    /// The ID of the course this upload belongs to
    pub belongs_to: Option<Uuid>,

    /// The ID of the prof that held the course this upload belongs to
    pub held_by: Option<Option<Uuid>>,
}

pub async fn handle_modify_upload(Json(request): Json<ModifyUploadRequest>) -> impl IntoResponse {
    let mut tx = (*DB_POOL.get().unwrap()).begin().await.unwrap();

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": "not implemented" })), // TODO
    )
}

#[derive(Debug, Deserialize)]
pub struct ModifyFileRequest {
    pub id: Uuid,
    pub name: Option<String>,
    pub mime_type: Option<String>,
    // The latest one should match the file's last modified date
    pub revision_at: Option<NaiveDateTime>,
    /// The ID of the upload this file belongs to
    pub upload_id: Option<Uuid>,
    pub approval_mod: Option<bool>,
}

pub async fn handle_modify_file(Json(request): Json<ModifyFileRequest>) -> impl IntoResponse {
    let mut tx = (*DB_POOL.get().unwrap()).begin().await.unwrap();

    let file = sqlx::query_as!(
        File,
        "
        SELECT
            id,
            name,
            mime_type,
            size,
            sha3_256,
            revision_at,
            upload_id,
            approval_uploader,
            approval_mod
        FROM
            files
        WHERE
            id = $1
        ",
        request.id
    )
    .fetch_one(&mut *tx)
    .await
    .context("Failed to get file")
    .unwrap();

    if request.name.is_some()
        || request.mime_type.is_some()
        || request.revision_at.is_some()
        || request.upload_id.is_some()
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "not implemented" })), // TODO
        );
    }

    if let Some(approval_mod) = request.approval_mod {
        sqlx::query!(
            "
            UPDATE
                files
            SET
                approval_mod = $1
            WHERE
                id = $2
            ",
            approval_mod,
            request.id
        )
        .execute(&mut *tx)
        .await
        .context("Failed to update file")
        .unwrap();
    }

    tx.commit().await.unwrap();

    (StatusCode::OK, Json(json!({ "success": true })))
}

pub async fn handle_get_all_uploads() -> impl IntoResponse {
    let mut tx = (*DB_POOL.get().unwrap()).begin().await.unwrap();

    let uploads = db::upload::get_all_uploads(&mut tx, None).await.unwrap();

    tx.commit().await.unwrap();

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "uploads": uploads,
        })),
    )
}

pub async fn handle_get_all_files() -> impl IntoResponse {
    let mut tx = (*DB_POOL.get().unwrap()).begin().await.unwrap();

    let files = db::file::get_all_files_and_join_upload(&mut tx)
        .await
        .unwrap();

    tx.commit().await.unwrap();

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "files": files,
        })),
    )
}

#[derive(Debug, Deserialize)]
pub struct GetFileAsModReq {
    pub file_id: Uuid,
}

/// Handles the actual download of a file to a client
async fn download_file_as_mod(
    Extension(current_user_id): Extension<Uuid>, // Get the user ID from the session
    Json(req): Json<GetFileAsModReq>,
) -> impl IntoResponse {
    let mut tx = (*DB_POOL.get().unwrap()).begin().await.unwrap();

    let maybe_file = db::file::get_file(&mut tx, req.file_id).await;

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

    // Deny access to mods if the uploader does not consent to the file being downloaded
    if !file.approval_uploader {
        log::info!(
            "Moderator {current_user_id} is unauthorized (uploader approval) to access file {}",
            file.id
        );

        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "success": false,
                "message": "Unauthorized",
            })),
        ));
    }

    // Prepare the download logic
    let do_download_to_user = async {
        let fs_file = tokio::fs::File::open(
            PathBuf::from("uploads")
                .join(&file.sha3_256[..2])
                .join(&file.sha3_256),
        )
        .await;

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

    log::info!(
        "Moderator {current_user_id} is authorized (mod) to access file {}",
        file.id
    );

    tx.commit().await.unwrap();

    do_download_to_user.await
}
