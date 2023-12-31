use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

use crate::data::SystemTransaction;

/// Calculates the amount of ECS the user has available to spend
///
/// Takes into account:
///
/// - ECS the user has earned (when a purchased upload is owned by the user)
/// - When a user has spent ECS on a purchase
/// - When the system has given/taken ECS from the user (see table `system_ec_transaction`)
pub async fn calculate_available_funds(
    db_pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: Uuid,
) -> anyhow::Result<f64> {
    // sqlx::query_file!("src/db/sql/get_available_ecs.sql", user_id,)
    //     .fetch_one(db_pool)
    //     .await
    //     .map(|row| row.ecs_available)
    //     .context("Failed to calculate available funds")
    Ok(42.0) // FIXME this darn thing doesn't work
}

pub async fn create_system_transaction(
    db_pool: &PgPool,
    transaction: SystemTransaction,
) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
            INSERT INTO system_ec_transaction (affected_user, transaction_date, delta_ec, reason)
            VALUES ($1, $2, $3, $4)
        "#,
        transaction.affected_user,
        transaction.transaction_date,
        transaction.delta_ec,
        transaction.reason,
    )
    .execute(db_pool)
    .await
    .context("Failed to create system transaction")?;

    Ok(())
}
