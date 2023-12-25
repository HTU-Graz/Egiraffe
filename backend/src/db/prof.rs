use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

use crate::data::Prof;

pub async fn get_prof(db_pool: &PgPool, prof_id: Uuid) -> anyhow::Result<Option<Prof>> {
    sqlx::query_as!(
        Prof,
        r#"
            SELECT prof.id,
                prof_name AS name
            FROM prof
            WHERE prof.id = $1
        "#,
        prof_id,
    )
    .fetch_optional(db_pool)
    .await
    .context("Failed to get courses")
}
