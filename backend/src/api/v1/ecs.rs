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
use sqlx::{PgPool, PgTransaction};
use uuid::Uuid;

use crate::{
    api::api_greeting,
    data::Prof,
    db::{self, DB_POOL},
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(api_greeting).post(api_greeting).put(api_greeting))
        .route("/get-user-balance", put(handle_get_user_balance))
        .route(
            "/create-system-transaction",
            put(handle_create_system_transaction),
        )
}

pub async fn handle_get_user_balance(Json(user_id): Json<Uuid>) -> impl IntoResponse {
    let mut tx = (*DB_POOL.get().unwrap()).begin().await.unwrap();

    let balance = db::ecs::calculate_available_funds(&mut tx, user_id).await;

    tx.commit().await.unwrap(); // TODO check if we really need a transaction here

    match balance {
        Ok(balance) => (
            StatusCode::OK,
            Json(json!({ "success": true, "balance": balance })),
        ),
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
    Json(req): Json<CreateSystemTransactionRequest>,
) -> impl IntoResponse {
    let mut tx = (*DB_POOL.get().unwrap()).begin().await.unwrap();

    log::info!("Creating system transaction: {:?}", req);

    let transaction = SystemTransaction {
        affected_user: req.user_id,
        transaction_date: chrono::Utc::now().naive_utc(),
        delta_ec: req.delta_ec,
        reason: req.reason,
    };

    let result = create_system_transaction(&mut tx, transaction).await;

    tx.commit().await.unwrap(); // TODO check if we really need a transaction here

    match result {
        Ok(_) => (StatusCode::OK, Json(json!({ "success": true }))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        ),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemTransaction {
    pub affected_user: Uuid,
    pub transaction_date: NaiveDateTime,

    /// The amount of ECS the user gained or lost
    pub delta_ec: i64,
    pub reason: Option<String>,
}

pub async fn create_system_transaction(
    mut tx: &mut PgTransaction<'_>,
    transaction: SystemTransaction,
) -> anyhow::Result<()> {
    sqlx::query!(
        "
        INSERT INTO
            system_ec_transactions (
                affected_user,
                transaction_date,
                delta_ec,
                reason
            )
        VALUES
            ($1, $2, $3, $4)
        ",
        transaction.affected_user,
        transaction.transaction_date,
        transaction.delta_ec,
        transaction.reason,
    )
    .execute(&mut **tx)
    .await
    .context("Failed to create system transaction")?;

    Ok(())
}
