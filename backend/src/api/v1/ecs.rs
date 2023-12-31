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

use crate::{
    api::api_greeting,
    data::{Prof, SystemTransaction},
    db, AppState,
};

pub fn routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(api_greeting).post(api_greeting).put(api_greeting))
        .route("/get-user-balance", put(handle_get_user_balance))
        .route(
            "/create-system-transaction",
            put(handle_create_system_transaction),
        )
}

pub async fn handle_get_user_balance(
    State(db_pool): State<AppState>,
    Json(user_id): Json<Uuid>,
) -> impl IntoResponse {
    let balance = db::ecs::calculate_available_funds(&db_pool, user_id).await;
    match balance {
        Ok(balance) => (StatusCode::OK, Json(json!({ "balance": balance }))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        ),
    }
}

#[derive(Deserialize, Debug)]
pub struct CreateSystemTransactionRequest {
    user_id: Uuid,
    delta_ec: i64,
    reason: Option<String>,
}

pub async fn handle_create_system_transaction(
    State(db_pool): State<AppState>,
    Json(req): Json<CreateSystemTransactionRequest>,
) -> impl IntoResponse {
    let transaction = SystemTransaction {
        affected_user: req.user_id,
        transaction_date: chrono::Utc::now().naive_utc(),
        delta_ec: req.delta_ec,
        reason: req.reason,
    };

    let result = db::ecs::create_system_transaction(&db_pool, transaction).await;
    match result {
        Ok(_) => (StatusCode::OK, Json(json!({}))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        ),
    }
}
