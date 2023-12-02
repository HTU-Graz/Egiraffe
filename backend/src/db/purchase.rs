use anyhow::Context;
use uuid::Uuid;

use crate::data::Purchase;

pub async fn get_purchase(
    db_pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: Uuid,
    upload_id: Uuid,
) -> anyhow::Result<Option<Purchase>> {
    sqlx::query_as!(
        Purchase,
        r#"
            SELECT user_id,
                upload_id,
                ecs_spent,
                purchase_date,
                rating
            FROM purchase
            WHERE user_id = $1
                AND upload_id = $2
        "#,
        user_id,
        upload_id,
    )
    .fetch_optional(db_pool)
    .await
    .context("Failed to get purchase")
}

pub async fn create_purchase(
    db_pool: &sqlx::Pool<sqlx::Postgres>,
    purchase: &Purchase,
) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
            INSERT INTO purchase (user_id, upload_id, ecs_spent, purchase_date, rating)
            VALUES ($1, $2, $3, $4, $5)
        "#,
        purchase.user_id,
        purchase.upload_id,
        purchase.ecs_spent,
        purchase.purchase_date,
        purchase.rating,
    )
    .execute(db_pool)
    .await
    .context("Failed to create purchase")?;

    Ok(())
}

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
    sqlx::query_file!("src/db/sql/get_available_ecs.sql", user_id,)
        .fetch_one(db_pool)
        .await
        .map(|row| row.ecs_available)
        .context("Failed to calculate available funds")
}
