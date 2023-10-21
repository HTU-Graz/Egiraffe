use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{
    api::api_greeting,
    data::{RedactedUser, Upload},
    db::{self, user::make_pwd_hash},
    AppState,
};

// Handles resource-modifying requests from authenticated users
pub fn routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(api_greeting).post(api_greeting).put(api_greeting))
        // .route("/courses", put(handle_get_courses))
        .route("/uploads", put(handle_do_upload))
        // .route("/universities", put(handle_get_universities))
        .route("/me", put(handle_do_me))
        .route("/file", put(handle_do_file))
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

    // TODO handle updating the description
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

        // 1. Create the upload
        let upload = {
            let DoUploadReq {
                name,
                price,
                belongs_to,
                held_by, // This actually is optional
                ..
            } = req;

            let (Some(name), Some(price), Some(belongs_to)) = (name, price, belongs_to) else {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "success": false,
                        "message": "Missing required fields",
                    })),
                );
            };

            let now = chrono::Utc::now().naive_utc();

            Upload {
                id: Uuid::new_v4(),
                name,
                description: String::new(),
                price,
                uploader: current_user_id,
                upload_date: now.clone(),
                last_modified_date: now,
                belongs_to,
                held_by,
            }
        };

        // 2. Insert the upload into the database
        let create_result = db::upload::create_upload(&db_pool, &upload).await;

        if create_result.is_ok() {
            log::info!("Upload created successfully, id: {}", upload.id);

            (
                StatusCode::OK,
                Json(json!({
                    "success": true,
                    "message": "Upload created successfully",
                    "upload": upload,
                })),
            )
        } else {
            log::error!("Failed to create upload: {}", create_result.unwrap_err());

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "message": "Failed to create upload",
                })),
            )
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DoMeReq {
    pub first_names: Option<String>,
    pub last_name: Option<String>,
    pub password: Option<String>,
    // TODO handle updating the email address

    // TODO handle updating the TOTP secret
    // totp_secret: Option<String>,
}

/// Handle updates to the current user's profile
///
/// Also: UwU
async fn handle_do_me(
    State(db_pool): State<AppState>,
    Extension(current_user_id): Extension<Uuid>, // Get the user ID from the session
    Json(req): Json<DoMeReq>,
) -> impl IntoResponse {
    // 1. Get the user from the database
    let maybe_user = db::user::get_user_by_id(&db_pool, current_user_id).await;
    let Ok(user) = maybe_user else {
        log::error!(
            "Failed to get user from database: {}",
            maybe_user.unwrap_err()
        );

        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": "Failed to get user from database",
            })),
        );
    };

    // TODO consider merging these `let else`s
    let Some(mut user) = user else {
        log::error!("Cannot modify: no such user: {current_user_id}");

        return (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "message": "No such user",
            })),
        );
    };

    // 2. Modify the user
    if let Some(first_names) = req.first_names {
        user.first_names = first_names.into();
    }

    if let Some(last_name) = req.last_name {
        user.last_name = last_name.into();
    }

    if let Some(password) = req.password {
        user.password_hash = make_pwd_hash(&password).into();
    }

    // 3. Update the user in the database
    let update_result = db::user::update_user(&db_pool, user.clone()).await;

    if update_result.is_ok() {
        log::info!("User updated successfully, id: {}", user.id);
    } else {
        log::error!("Failed to update user: {}", update_result.unwrap_err());

        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": "Failed to update user",
            })),
        );
    }

    // 4. Return the updated user
    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "message": "User retrieved successfully",
            "user": RedactedUser::from(user),
        })),
    )
}

#[derive(Debug, Serialize, Deserialize)]
struct DoFileReq {
    /// The ID of the upload this file belongs to
    upload_id: Uuid,

    /// The file's name
    name: String,

    /// The file's MIME type
    mime_type: String,

    /// The file's contents, byte buffer
    contents: Vec<u8>,
}

async fn handle_do_file(
    State(db_pool): State<AppState>,
    Extension(current_user_id): Extension<Uuid>, // Get the user ID from the session
    mut multipart: Multipart,
) -> impl IntoResponse {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_owned();
        let filename = field.file_name().unwrap().to_owned();
        let content_type = field.content_type().unwrap().to_owned();
        let mut bytes = Vec::new();
        while let Some(chunk) = field.chunk().await.unwrap() {
            bytes.extend_from_slice(&chunk);
        }
        println!("{} {} {} {}", name, filename, content_type, bytes.len());
    }

    // TODO finish this

    todo!("Complete file upload");

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "message": "File uploaded successfully",
        })),
    )
}
