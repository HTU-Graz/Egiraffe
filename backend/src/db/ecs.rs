use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_user_balance(db_pool: &PgPool, user_id: Uuid) -> anyhow::Result<i64> {
    todo!();

    // let balance = sqlx::query!(
    //     r#"
    //         SELECT ...
    //         JOIN something
    //         FROM users
    //         WHERE id = $1
    //     "#,
    //     user_id,
    // )
    // .fetch_one(db_pool)
    // .await
    // .context("Failed to get user balance")?
    // .balance;

    // Ok(balance)
}

pub async fn create_system_transaction(
    db_pool: &PgPool,
    user_id: Uuid,
    amount: i64,
    reason: &str,
) -> anyhow::Result<()> {
    todo!();

    // sqlx::query!(
    //     r#"
    //         INSERT INTO transactions (user_id, amount, reason)
    //         VALUES ($1, $2, $3)
    //     "#,
    //     user_id,
    //     amount,
    //     reason,
    // )
    // .execute(db_pool)
    // .await
    // .context("Failed to create system transaction")?;

    // Ok(())
}
