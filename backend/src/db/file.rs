use anyhow::Context;

use crate::data::File;

pub async fn create_file(db_pool: &sqlx::Pool<sqlx::Postgres>, file: &File) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
            INSERT INTO file (
                    id,
                    name,
                    mime_type,
                    size,
                    upload_id
                )
            VALUES ($1, $2, $3, $4, $5)
        "#,
        file.id,
        file.name,
        file.mime_type,
        file.size,
        file.upload_id,
    )
    .execute(db_pool)
    .await
    .context("Failed to create file")?;

    Ok(())
}
