use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

use crate::data::Prof;

pub async fn get_prof(db_pool: &PgPool, prof_id: Uuid) -> anyhow::Result<Option<Prof>> {
    sqlx::query_as!(
        Prof,
        "
        SELECT
            profs.id,
            prof_name AS name
        FROM
            profs
        WHERE
            profs.id = $1
        ",
        prof_id,
    )
    .fetch_optional(db_pool)
    .await
    .context("Failed to get courses")
}

pub async fn get_profs(db_pool: &PgPool) -> anyhow::Result<Vec<Prof>> {
    let profs = sqlx::query_as!(
        Prof,
        "
        SELECT
            profs.id,
            prof_name AS name
        FROM
            profs
        "
    )
    .fetch_all(db_pool)
    .await
    .context("Failed to get courses")?;

    Ok(profs)
}

pub async fn create_prof(db_pool: &PgPool, prof: &Prof) -> anyhow::Result<()> {
    sqlx::query!(
        "
        INSERT INTO
            profs (id, prof_name)
        VALUES
            ($1, $2)
        ",
        prof.id,
        prof.name,
    )
    .execute(db_pool)
    .await
    .context("Failed to create prof")?;

    Ok(())
}

pub async fn update_prof(db_pool: &PgPool, prof: &Prof) -> anyhow::Result<()> {
    sqlx::query!(
        "
        UPDATE
            profs
        SET
            prof_name = $1
        WHERE
            id = $2
        ",
        prof.name,
        prof.id,
    )
    .execute(db_pool)
    .await
    .context("Failed to update prof")?;

    Ok(())
}
