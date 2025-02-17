use std::path::PathBuf;

use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
    Extension, Json, Router,
};
use futures::{io, TryStreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::io::AsyncWriteExt;
use tokio_util::io::StreamReader;
use uuid::Uuid;

use crate::{
    api::api_greeting,
    data::{File, Purchase, RedactedUser, Upload},
    db::{self, user::make_pwd_hash, DB_POOL},
    util::bad_request,
};

// Handles resource-modifying requests from authenticated users
pub fn routes() -> Router {
    Router::new()
        .route("/", get(api_greeting).post(api_greeting).put(api_greeting))
        // .route("/courses", put(handle_get_courses))
        .route("/upload", put(handle_do_upload))
        // .route("/universities", put(handle_get_universities))
        .route("/me", put(handle_do_me))
        .route("/file", put(handle_do_file))
        .route("/purchase", put(handle_do_purchase))
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

    // TODO document this
    pub associated_date: Option<chrono::NaiveDateTime>,
    // TODO impl category
}

async fn handle_do_upload(
    Extension(current_user_id): Extension<Uuid>, // Get the user ID from the session
    Json(req): Json<DoUploadReq>,
) -> impl IntoResponse {
    let db_pool = *DB_POOL.get().unwrap();

    // log::info!("Create/alter upload for course {}", req.belongs_to.unwrap_or("default"));

    dbg!(&req);

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
                description,
                price,
                belongs_to,
                held_by,         // This actually is optional
                associated_date, // This is optional too
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
                description: description.unwrap_or(String::new()),
                price,
                uploader: current_user_id,
                upload_date: now.clone(),
                last_modified_date: now,
                associated_date,
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
    Extension(current_user_id): Extension<Uuid>, // Get the user ID from the session
    Json(req): Json<DoMeReq>,
) -> impl IntoResponse {
    let db_pool = *DB_POOL.get().unwrap();

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
}

async fn handle_do_file(
    Extension(current_user_id): Extension<Uuid>, // Get the user ID from the session
    mut multipart: Multipart,
) -> impl IntoResponse {
    let db_pool = *DB_POOL.get().unwrap();

    // 0. Get the form fields
    // 0.a. Get the upload ID
    let Some(field) = multipart.next_field().await.unwrap() else {
        return bad_request("Missing some form field");
    };
    if field.name().unwrap().to_owned() != "upload_id" {
        return bad_request("Invalid form field name, expected \"upload_id\"");
    }

    let upload_id = Uuid::parse_str(&field.text().await.unwrap()).unwrap();

    // 0.b. Get the file name & MIME type
    let Some(field) = multipart.next_field().await.unwrap() else {
        return bad_request("Missing some form field");
    };
    if field.name().unwrap().to_owned() != "file" {
        return bad_request("Invalid form field name, expected \"file\"");
    }

    let name = field.file_name().unwrap().to_owned();
    let mime_type = field.content_type().unwrap().to_owned();

    let upload_req = DoFileReq {
        upload_id,
        name,
        mime_type,
    };

    // 2. Begin reading & writing the file's contents; and generate file metadata
    let file_id = Uuid::new_v4();

    // 3. Write the file to disk & check for collisions
    std::fs::create_dir_all("uploads").unwrap();
    let path = PathBuf::from("uploads").join(file_id.to_string());

    if path.exists() {
        return bad_request("File already exists; please try again");
    }

    let mut fs_file = tokio::fs::File::create(&path).await.unwrap();
    let body_with_io_error = field.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
    let body_reader = StreamReader::new(body_with_io_error);
    futures::pin_mut!(body_reader);

    // TODO consider moving this till after the database stuff
    let file_writing_future = tokio::io::copy(&mut body_reader, &mut fs_file);

    // 1. Get the upload from the database
    let maybe_upload = db::upload::get_upload_by_id(&db_pool, upload_req.upload_id).await;
    let Ok(upload) = maybe_upload else {
        log::error!(
            "Failed to get upload from database: {}",
            maybe_upload.unwrap_err()
        );

        // Stop writing the file to disk & delete it
        let _ = file_writing_future.await;
        let _ = tokio::fs::remove_file(path).await;

        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": "Failed to get upload from database",
            })),
        );
    };

    let Some(upload) = upload else {
        log::error!(
            "Cannot modify: no such upload: {id}",
            id = upload_req.upload_id
        );

        // Stop writing the file to disk & delete it
        let _ = file_writing_future.await;
        let _ = tokio::fs::remove_file(path).await;

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

        // Stop writing the file to disk & delete it
        let _ = file_writing_future.await;
        let _ = tokio::fs::remove_file(path).await;

        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "success": false,
                "message": "User is not allowed to modify this upload",
            })),
        );
    }

    // 3. Finish writing the file to disk
    let bytes_written = file_writing_future.await.unwrap();
    let file = File {
        id: file_id,
        name: upload_req.name,
        mime_type: upload_req.mime_type,
        size: bytes_written as i64,
        upload_id: upload_req.upload_id,
        revision_at: chrono::Utc::now().naive_utc(), // FIXME update the upload's last modified date to this timestamp
        approval_uploader: true, // TODO consider making this user-configurable to allow for "draft uploads"
        approval_mod: false,
    };

    // 4. Persist the file in the database
    let maybe_file = db::file::create_file(&db_pool, &file).await;
    if maybe_file.is_err() {
        log::error!("Failed to create file: {}", maybe_file.unwrap_err());

        // Delete the file
        let _ = tokio::fs::remove_file(path).await;

        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": "Failed to create file",
            })),
        );
    };

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "message": "File uploaded successfully",
            "file": file,
        })),
    )
}

#[derive(Debug, Serialize, Deserialize)]
struct DoPurchaseReq {
    /// The ID of the upload this file belongs to
    upload_id: Uuid,
}

async fn handle_do_purchase(
    Extension(current_user_id): Extension<Uuid>, // Get the user ID from the session
    Json(req): Json<DoPurchaseReq>,
) -> impl IntoResponse {
    let db_pool = *DB_POOL.get().unwrap();

    // 1. Get the upload from the database
    let maybe_upload = db::upload::get_upload_by_id(&db_pool, req.upload_id).await;
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
        log::error!("Cannot purchase: no such upload: {id}", id = req.upload_id);

        return (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "message": "No such upload",
            })),
        );
    };

    // // 2. Get the current user from the database
    // let maybe_user = db::user::get_user_by_id(&db_pool, current_user_id).await;
    // let Ok(Some(user)) = maybe_user else {
    //     log::error!(
    //         "Failed to get user from database: {}",
    //         maybe_user.unwrap_err()
    //     );
    //
    //     return (
    //         StatusCode::INTERNAL_SERVER_ERROR,
    //         Json(json!({
    //             "success": false,
    //             "message": "Failed to get user from database",
    //         })),
    //     );
    // };

    // 3. Check if the user has already purchased this upload
    let maybe_purchase = db::purchase::get_purchase(&db_pool, current_user_id, req.upload_id).await;
    let Ok(purchase) = maybe_purchase else {
        log::error!(
            "Failed to get purchase from database: {}",
            maybe_purchase.unwrap_err()
        );

        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": "Failed to get purchase from database",
            })),
        );
    };

    if purchase.is_some() {
        log::error!(
            "Cannot purchase: user ({current_user_id}) has already purchased this upload ({})",
            upload.uploader
        );

        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "message": "User has already purchased this upload",
            })),
        );
    }

    // TODO 4. Check if the user has enough ECS to purchase this upload

    // 5. Create the purchase
    let purchase = Purchase {
        user_id: current_user_id,
        upload_id: req.upload_id,
        ecs_spent: upload.price,
        purchase_date: chrono::Utc::now().naive_utc().clone(),
        rating: None,
    };

    // 6. Persist the purchase in the database
    let create_result = db::purchase::create_purchase(&db_pool, &purchase).await;
    if create_result.is_err() {
        log::error!("Failed to create purchase: {}", create_result.unwrap_err());

        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": "Failed to create purchase",
            })),
        );
    };

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "message": "Purchase successful",
            "purchase": purchase,
            "upload": upload,
        })),
    )
}
