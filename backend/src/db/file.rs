use crate::data::File;

pub async fn create_file(db_pool: &sqlx::Pool<sqlx::Postgres>, file: &File) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
            INSERT INTO file (
                    id,
                    name,
                    mime_type,
                    size,
                    upload_id,
                    revision_at
                )
            VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        file.id,
        file.name,
        file.mime_type,
        file.size,
        file.upload_id,
        file.revision_at
    )
    .execute(db_pool)
    .await
    // .unwrap();
    // .context("Failed to create file")?;
    ?;

    Ok(())
}
