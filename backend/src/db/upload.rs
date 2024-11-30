use anyhow::Context;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::data::Upload;

use super::SortOrder;

#[derive(Debug, Deserialize)]
pub struct Sorting {
    pub order: SortOrder,
    pub by: SortBy,
}

#[derive(Debug, Deserialize)]
pub enum SortBy {
    Name,
    UploadDate,
    Date,
    Rating,
}

impl Default for Sorting {
    fn default() -> Self {
        Self {
            order: SortOrder::Descending,
            by: SortBy::Date,
        }
    }
}

pub async fn get_uploads_of_course(
    db_pool: &PgPool,
    course_id: Uuid,
    sorting: Option<Sorting>,
) -> anyhow::Result<Vec<Upload>> {
    // TODO: implement sorting
    let _sorting = sorting.unwrap_or_default();

    sqlx::query_as!(
        Upload,
        "
        SELECT
            uploads.id,
            upload_name AS name,
            description,
            price,
            uploader,
            upload_date,
            last_modified_date,
            belongs_to,
            held_by
        FROM
            uploads
            INNER JOIN courses ON uploads.belongs_to = courses.id
        WHERE
            courses.id = $1
        ",
        course_id,
    )
    .fetch_all(db_pool)
    .await
    .context("Failed to get courses")
}

pub async fn get_all_uploads(
    db_pool: &PgPool,
    sorting: Option<Sorting>,
) -> anyhow::Result<Vec<Upload>> {
    // TODO: implement sorting
    let _sorting = sorting.unwrap_or_default();

    sqlx::query_as!(
        Upload,
        "
        SELECT
            uploads.id,
            upload_name AS name,
            description,
            price,
            uploader,
            upload_date,
            last_modified_date,
            belongs_to,
            held_by
        FROM
            uploads
            INNER JOIN courses ON uploads.belongs_to = courses.id
        ",
    )
    .fetch_all(db_pool)
    .await
    .context("Failed to get courses")
}

pub async fn get_upload_by_id(db_pool: &PgPool, upload_id: Uuid) -> anyhow::Result<Option<Upload>> {
    sqlx::query_as!(
        Upload,
        "
        SELECT
            uploads.id,
            upload_name AS name,
            description,
            price,
            uploader,
            upload_date,
            last_modified_date,
            belongs_to,
            held_by
        FROM
            uploads
        WHERE
            uploads.id = $1
        ",
        upload_id,
    )
    .fetch_optional(db_pool)
    .await
    .context("Failed to get upload by ID")
}

pub async fn get_upload_by_id_and_join_course(
    db_pool: &PgPool,
    upload_id: Uuid,
) -> anyhow::Result<Option<(Upload, String)>> {
    let row = sqlx::query!(
        "
        SELECT
            uploads.id,
            upload_name AS name,
            description,
            price,
            uploader,
            upload_date,
            last_modified_date,
            belongs_to,
            held_by,
            course_name AS course_name
        FROM
            uploads
            INNER JOIN courses ON uploads.belongs_to = courses.id
        WHERE
            uploads.id = $1
        ",
        upload_id,
    )
    .fetch_optional(db_pool)
    .await
    .context("Failed to get upload by ID");

    match row {
        Ok(Some(row)) => Ok(Some((
            Upload {
                id: row.id,
                name: row.name,
                description: row.description,
                price: row.price,
                uploader: row.uploader,
                upload_date: row.upload_date,
                last_modified_date: row.last_modified_date,
                belongs_to: row.belongs_to,
                held_by: row.held_by,
            },
            row.course_name,
        ))),
        Ok(None) => Ok(None),
        Err(err) => Err(err).context("Failed to get upload by ID"),
    }
}

pub async fn update_upload(
    db_pool: &sqlx::Pool<sqlx::Postgres>,
    upload: &Upload,
) -> anyhow::Result<()> {
    sqlx::query!(
        "
        UPDATE
            uploads
        SET
            upload_name = $1,
            description = $2,
            price = $3,
            last_modified_date = $4
        WHERE
            id = $5
        ",
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
        "
        INSERT INTO
            uploads (
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
        VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        ",
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
