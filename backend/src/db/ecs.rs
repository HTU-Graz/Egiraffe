use anyhow::Context;
use sqlx::PgTransaction;
use uuid::Uuid;

/// Calculates the amount of ECS the user has available to spend
///
/// Takes into account:
///
/// - ECS the user has earned (when a purchased upload is owned by the user)
/// - When a user has spent ECS on a purchase
/// - When the system has given/taken ECS from the user (see table `system_ec_transaction`)
pub async fn calculate_available_funds(
    mut tx: &mut PgTransaction<'_>,
    user_id: Uuid,
) -> anyhow::Result<f64> {
    // FIXME figure out what data type should be returned for this query
    // sqlx::query_file!("src/db/sql/get_available_ecs.sql", user_id,)
    //     .fetch_one(&mut **tx)
    //     .await
    //     .map(|row| row.ecs_available)
    //     .context("Failed to calculate available funds")
    Ok(42.0) // FIXME this darn thing doesn't work
}
