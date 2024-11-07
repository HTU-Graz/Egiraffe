use anyhow::Context;
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
    Json, Router,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    api::api_greeting,
    data::{Prof, RedactedUser},
    db::{self, DB_POOL},
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(api_greeting).post(api_greeting).put(api_greeting))
        .route("/get-users", put(handle_get_users))
}

pub async fn handle_get_users() -> impl IntoResponse {
    let db_pool = *DB_POOL.get().unwrap();

    // Select totp_secret as totp_enabled (check if it's null or if the string has length > 0)
    // let users = sqlx::query_as!(
    let users: anyhow::Result<Vec<(Uuid, Option<String>, Option<String>, bool, i16)>> =
        sqlx::query_as(
            // RedactedUser,
            r#"
            SELECT
                id,
                first_names,
                last_name,
                true AS totp_enabled,
                user_role
            FROM
                "user"
            "#,
        )
        .fetch_all(&*db_pool)
        .await
        .context("Failed to fetch users");

    let Ok(users) = users else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Failed to fetch users" })),
        );
    };

    return (StatusCode::OK, Json(json!({ "users": users })));
}
