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
        "
        SELECT
            user_id,
            upload_id,
            ecs_spent,
            purchase_date,
            rating
        FROM
            purchases
        WHERE
            user_id = $1
            AND upload_id = $2
        ",
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
        "
        INSERT INTO
            purchases (
                user_id,
                upload_id,
                ecs_spent,
                purchase_date,
                rating
            )
        VALUES
            ($1, $2, $3, $4, $5)
        ",
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
