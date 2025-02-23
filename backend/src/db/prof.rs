use anyhow::Context;
use sqlx::PgTransaction;
use uuid::Uuid;

use crate::data::Prof;

pub async fn get_prof(
    mut tx: &mut PgTransaction<'_>,
    prof_id: Uuid,
) -> anyhow::Result<Option<Prof>> {
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
    .fetch_optional(&mut **tx)
    .await
    .context("Failed to get courses")
}

pub async fn get_profs(mut tx: &mut PgTransaction<'_>) -> anyhow::Result<Vec<Prof>> {
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
    .fetch_all(&mut **tx)
    .await
    .context("Failed to get courses")?;

    Ok(profs)
}

pub async fn create_prof(mut tx: &mut PgTransaction<'_>, prof: &Prof) -> anyhow::Result<()> {
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
    .execute(&mut **tx)
    .await
    .context("Failed to create prof")?;

    Ok(())
}

pub async fn update_prof(mut tx: &mut PgTransaction<'_>, prof: &Prof) -> anyhow::Result<()> {
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
    .execute(&mut **tx)
    .await
    .context("Failed to update prof")?;

    Ok(())
}
