//! Content moderation API endpoints

use anyhow::Context;
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
    Json, Router,
};
use chrono::NaiveDateTime;
use serde::Deserialize;
use serde_json::json;
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
    let db_pool = *DB_POOL.get().unwrap();

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
    let db_pool = *DB_POOL.get().unwrap();

    let mut tx = db_pool.begin().await.unwrap();

    let file = sqlx::query_as!(
        File,
        "
        SELECT
            id,
            name,
            mime_type,
            size,
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
    let db_pool = *DB_POOL.get().unwrap();

    let uploads = db::upload::get_all_uploads(&db_pool, None).await.unwrap();

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "uploads": uploads,
        })),
    )
}

pub async fn handle_get_all_files() -> impl IntoResponse {
    let db_pool = *DB_POOL.get().unwrap();

    let files = db::file::get_all_files_and_join_upload(&db_pool)
        .await
        .unwrap();

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "files": files,
        })),
    )
}
