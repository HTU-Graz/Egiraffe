use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

use crate::data::Upload;

pub async fn get_uploads_of_course(
    db_pool: &PgPool,
    course_id: Uuid,
) -> anyhow::Result<Vec<Upload>> {
    sqlx::query_as!(
        Upload,
        r#"
            SELECT upload.id,
                upload_name AS name,
                description,
                price,
                uploader,
                upload_date,
                last_modified_date,
                belongs_to,
                held_by
            FROM upload
                INNER JOIN course ON upload.belongs_to = course.id
            WHERE course.id = $1
        "#,
        course_id,
    )
    .fetch_all(db_pool)
    .await
    .context("Failed to get courses")
}

pub async fn get_upload_by_id(db_pool: &PgPool, upload_id: Uuid) -> anyhow::Result<Option<Upload>> {
    sqlx::query_as!(
        Upload,
        r#"
            SELECT upload.id,
                upload_name AS name,
                description,
                price,
                uploader,
                upload_date,
                last_modified_date,
                belongs_to,
                held_by
            FROM upload
            WHERE upload.id = $1
        "#,
        upload_id,
    )
    .fetch_optional(db_pool)
    .await
    .context("Failed to get upload by ID")
}

pub async fn update_upload(
    db_pool: &sqlx::Pool<sqlx::Postgres>,
    upload: &Upload,
) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
            UPDATE upload
            SET upload_name = $1,
                description = $2,
                price = $3,
                last_modified_date = $4
            WHERE id = $5
        "#,
        upload.name,
        upload.description,
        upload.price,
        upload.last_modified_date,
        upload.id,
    )
    .execute(db_pool)
    .await
    .context("Failed to update upload")?;

    Ok(())
}

pub async fn create_upload(
    db_pool: &sqlx::Pool<sqlx::Postgres>,
    upload: &Upload,
) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
            INSERT INTO upload (
                    id,
                    upload_name,
                    description,
                    price,
                    uploader,
                    upload_date,
                    last_modified_date,
                    belongs_to,
                    held_by
                )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
        upload.id,
        upload.name,
        upload.description,
        upload.price,
        upload.uploader,
        upload.upload_date,
        upload.last_modified_date,
        upload.belongs_to,
        upload.held_by,
    )
    .execute(db_pool)
    .await
    .context("Failed to create upload")?;

    Ok(())
}
