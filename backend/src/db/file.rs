use uuid::Uuid;

use crate::data::{File, Upload};

pub async fn create_file(db_pool: &sqlx::Pool<sqlx::Postgres>, file: &File) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
            INSERT INTO file (
                    id,
                    name,
                    mime_type,
                    size,
                    upload_id,
                    revision_at,
                    approval_uploader,
                    approval_mod
                )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
        file.id,
        file.name,
        file.mime_type,
        file.size,
        file.upload_id,
        file.revision_at,
        file.approval_uploader,
        file.approval_mod
    )
    .execute(db_pool)
    .await
    // .unwrap();
    // .context("Failed to create file")?;
    ?;

    Ok(())
}

pub async fn get_file(db_pool: &sqlx::Pool<sqlx::Postgres>, id: Uuid) -> anyhow::Result<File> {
    let file = sqlx::query_as!(
        File,
        r#"
            SELECT
                id,
                name,
                mime_type,
                size,
                revision_at,
                upload_id,
                approval_uploader,
                approval_mod
            FROM file
            WHERE id = $1
        "#,
        id
    )
    .fetch_one(db_pool)
    .await
    // .unwrap();
    // .context("Failed to get file")?;
    ?;

    Ok(file)
}

pub async fn get_files_of_upload(
    db_pool: &sqlx::Pool<sqlx::Postgres>,
    upload_id: Uuid,
) -> anyhow::Result<Vec<File>> {
    let files = sqlx::query_as!(
        File,
        r#"
            SELECT
                id,
                name,
                mime_type,
                size,
                revision_at,
                upload_id,
                approval_uploader,
                approval_mod
            FROM file
            WHERE upload_id = $1
        "#,
        upload_id
    )
    .fetch_all(db_pool)
    .await
    // .unwrap();
    // .context("Failed to get files of upload")?;
    ?;

    Ok(files)
}

pub async fn get_files_and_join_upload(
    db_pool: &sqlx::Pool<sqlx::Postgres>,
    upload_id: Uuid,
) -> anyhow::Result<Vec<(File, Upload)>> {
    let file_upload_joins = sqlx::query!(
        r#"
            SELECT
                file.id AS file_id,
                file.name,
                file.mime_type,
                file.size,
                file.revision_at,
                file.approval_uploader,
                file.approval_mod,
                upload.id AS upload_id,
                upload.upload_name,
                upload.description,
                upload.price,
                upload.uploader,
                upload.upload_date,
                upload.last_modified_date,
                upload.belongs_to,
                upload.held_by
            FROM file
            INNER JOIN upload
                ON file.upload_id = upload.id
            WHERE upload_id = $1
        "#,
        upload_id
    )
    .fetch_all(db_pool)
    .await?
    .into_iter()
    .map(|row| {
        (
            File {
                id: row.file_id,
                name: row.name,
                mime_type: row.mime_type,
                size: row.size,
                revision_at: row.revision_at,
                upload_id: row.upload_id,
                approval_uploader: row.approval_uploader,
                approval_mod: row.approval_mod,
            },
            Upload {
                id: row.upload_id,
                name: row.upload_name,
                description: row.description,
                price: row.price,
                uploader: row.uploader,
                upload_date: row.upload_date,
                last_modified_date: row.last_modified_date,
                belongs_to: row.belongs_to,
                held_by: row.held_by,
            },
        )
    })
    .collect();

    Ok(file_upload_joins)
}

pub async fn get_all_files_and_join_upload(
    db_pool: &sqlx::Pool<sqlx::Postgres>,
) -> anyhow::Result<Vec<(File, Upload)>> {
    let file_upload_joins = sqlx::query!(
        r#"
            SELECT
                file.id AS file_id,
                file.name,
                file.mime_type,
                file.size,
                file.revision_at,
                file.approval_uploader,
                file.approval_mod,
                upload.id AS upload_id,
                upload.upload_name,
                upload.description,
                upload.price,
                upload.uploader,
                upload.upload_date,
                upload.last_modified_date,
                upload.belongs_to,
                upload.held_by
            FROM file
            INNER JOIN upload
                ON file.upload_id = upload.id
        "#,
    )
    .fetch_all(db_pool)
    .await?
    .into_iter()
    .map(|row| {
        (
            File {
                id: row.file_id,
                name: row.name,
                mime_type: row.mime_type,
                size: row.size,
                revision_at: row.revision_at,
                upload_id: row.upload_id,
                approval_uploader: row.approval_uploader,
                approval_mod: row.approval_mod,
            },
            Upload {
                id: row.upload_id,
                name: row.upload_name,
                description: row.description,
                price: row.price,
                uploader: row.uploader,
                upload_date: row.upload_date,
                last_modified_date: row.last_modified_date,
                belongs_to: row.belongs_to,
                held_by: row.held_by,
            },
        )
    })
    .collect();

    Ok(file_upload_joins)
}

pub async fn get_upload_of_file(
    db_pool: &sqlx::Pool<sqlx::Postgres>,
    file_id: Uuid,
) -> anyhow::Result<Upload> {
    let upload = sqlx::query_as!(
        Upload,
        r#"
            SELECT
                id,
                upload_name AS name,
                description,
                price,
                uploader,
                upload_date,
                last_modified_date,
                belongs_to,
                held_by
            FROM upload
            WHERE id = (
                SELECT upload_id
                FROM file
                WHERE id = $1
            )
        "#,
        file_id
    )
    .fetch_one(db_pool)
    .await
    // .unwrap();
    // .context("Failed to get upload of file")?;
    ?;

    Ok(upload)
}
