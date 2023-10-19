use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{api::api_greeting, db, AppState};

// Handles resource-modifying requests from authenticated users
pub fn routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(api_greeting).post(api_greeting).put(api_greeting))
        // .route("/courses", put(handle_get_courses))
        .route("/uploads", put(handle_do_upload))
    // .route("/universities", put(handle_get_universities))
    // .route("/me", put(handle_get_me))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DoUploadReq {
    /// The ID of an existing upload, or `None` if this is a new upload
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<i16>,

    /// The ID of the course this upload belongs to
    pub belongs_to: Option<Uuid>,

    /// The ID of the prof that held the course this upload belongs to
    pub held_by: Option<Uuid>,
}

async fn handle_do_upload(
    State(db_pool): State<AppState>,
    Extension(current_user_id): Extension<Uuid>, // Get the user ID from the session
    Json(req): Json<DoUploadReq>,
) -> impl IntoResponse {
    // log::info!("Create/alter upload for course {}", req.belongs_to.unwrap_or("default"));

    // 0. Check if a new upload is being created or an existing one is being modified
    if let Some(id) = req.id {
        // 1. Get the upload from the database
        let maybe_upload = db::upload::get_upload_by_id(&db_pool, id).await;
        let Ok(upload) = maybe_upload else {
            log::error!(
                "Failed to get upload from database: {}",
                maybe_upload.unwrap_err()
            );

            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "message": "Failed to get upload from database",
                })),
            );
        };

        let Some(upload) = upload else {
            log::error!("Cannot modify: no such upload: {id}");

            return (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "success": false,
                    "message": "No such upload",
                })),
            );
        };

        // 2. Check if the user is allowed to modify this upload
        if upload.uploader != current_user_id {
            log::error!(
                "Cannot modify: user ({current_user_id}) is not the uploader ({})",
                upload.uploader
            );

            return (
                StatusCode::FORBIDDEN,
                Json(json!({
                    "success": false,
                    "message": "User is not allowed to modify this upload",
                })),
            );
        }

        // 3. Modify the upload
        let mut upload = upload;

        if let Some(name) = req.name {
            upload.name = name;
        }

        if let Some(description) = req.description {
            upload.description = description;
        }

        if let Some(price) = req.price {
            upload.price = price;
        }

        if let Some(belongs_to) = req.belongs_to {
            upload.belongs_to = belongs_to;
        }

        if let Some(held_by) = req.held_by {
            upload.held_by = Some(held_by);
        }

        upload.last_modified_date = chrono::Utc::now().naive_utc();

        // 4. Update the upload in the database
        let update_result = db::upload::update_upload(&db_pool, &upload).await;

        if update_result.is_ok() {
            log::info!("Upload updated successfully, id: {}", upload.id);

            (
                StatusCode::OK,
                Json(json!({
                    "success": true,
                    "message": "Upload updated successfully",
                })),
            )
        } else {
            log::error!("Failed to update upload: {}", update_result.unwrap_err());

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "message": "Failed to update upload",
                })),
            )
        }
    } else {
        // Case 2: new upload is being created
        // 1. Check if the user is allowed to create a new upload

        todo!("Case 2: new upload is being created");
    }
}
