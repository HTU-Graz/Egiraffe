use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::data::{File, Upload, UploadType};

pub async fn create_file(db_pool: &sqlx::Pool<sqlx::Postgres>, file: &File) -> anyhow::Result<()> {
    sqlx::query!(
        "
        INSERT INTO
            files (
                id,
                name,
                mime_type,
                size,
                sha3_256,
                upload_id,
                revision_at,
                approval_uploader,
                approval_mod
            )
        VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        ",
        file.id,
        file.name,
        file.mime_type,
        file.size,
        file.sha3_256,
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
        "
        SELECT
            id,
            name,
            mime_type,
            size,
            sha3_256,
            revision_at,
            upload_id,
            approval_uploader,
            approval_mod
        FROM
            files
        WHERE
            id = $1
        ",
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
        "
        SELECT
            id,
            name,
            mime_type,
            size,
            sha3_256,
            revision_at,
            upload_id,
            approval_uploader,
            approval_mod
        FROM
            files
        WHERE
            upload_id = $1
        ",
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
    let file_upload_joins: Vec<FileUpload> = sqlx::query_as!(
        FileUpload,
        r#"
        SELECT
            files.id AS file_id,
            files.name AS file_name,
            files.mime_type,
            files.size,
            files.sha3_256,
            files.revision_at,
            files.approval_uploader,
            files.approval_mod,
            uploads.id AS upload_id,
            uploads.upload_name,
            uploads.description,
            uploads.price,
            uploads.uploader,
            uploads.upload_date,
            uploads.last_modified_date,
            uploads.associated_date,
            uploads.upload_type AS "upload_type: _",
            uploads.belongs_to,
            uploads.held_by
        FROM
            files
            INNER JOIN uploads ON files.upload_id = uploads.id
        WHERE
            upload_id = $1
        "#,
        upload_id
    )
    .fetch_all(db_pool)
    .await?;

    let file_upload_joins = file_upload_joins
        .into_iter()
        .map(|row| {
            (
                File {
                    id: row.file_id,
                    name: row.file_name,
                    mime_type: row.mime_type,
                    size: row.size,
                    sha3_256: row.sha3_256,
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
                    associated_date: row.associated_date,
                    upload_type: row.upload_type,
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
    let file_upload_joins = sqlx::query_as!(
        FileUpload,
        r#"
        SELECT
            files.id AS file_id,
            files.name AS file_name,
            files.mime_type,
            files.size,
            files.sha3_256,
            files.revision_at,
            files.approval_uploader,
            files.approval_mod,
            uploads.id AS upload_id,
            uploads.upload_name,
            uploads.description,
            uploads.price,
            uploads.uploader,
            uploads.upload_date,
            uploads.last_modified_date,
            uploads.associated_date,
            uploads.upload_type AS "upload_type: _",
            uploads.belongs_to,
            uploads.held_by
        FROM
            files
            INNER JOIN uploads ON files.upload_id = uploads.id
        "#,
    )
    .fetch_all(db_pool)
    .await?
    .into_iter()
    .map(|row| {
        (
            File {
                id: row.file_id,
                name: row.file_name,
                mime_type: row.mime_type,
                size: row.size,
                sha3_256: row.sha3_256,
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
                associated_date: row.associated_date,
                upload_type: row.upload_type,
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
            associated_date,
            upload_type AS "upload_type: _",
            belongs_to,
            held_by
        FROM
            uploads
        WHERE
            id = (
                SELECT
                    upload_id
                FROM
                    files
                WHERE
                    id = $1
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

// Oida
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FileUpload {
    pub file_id: Uuid,
    pub file_name: String,
    pub mime_type: String,
    pub size: i64,
    pub sha3_256: String,
    // The latest one should match the file's last modified date
    pub revision_at: NaiveDateTime,
    /// The ID of the upload this file belongs to
    pub upload_id: Uuid,
    pub approval_uploader: bool,
    pub approval_mod: bool,
    //
    //
    //
    // pub upload_id: Uuid,
    pub upload_name: String,
    pub description: String,
    pub price: i16,
    pub uploader: Uuid, // TODO consider adding resolved values for faster API times
    pub upload_date: NaiveDateTime,
    pub last_modified_date: NaiveDateTime,

    /// The date associated with the upload, e.g. the date of the exam (nullable)
    pub associated_date: Option<NaiveDateTime>,

    pub upload_type: UploadType,

    /// The ID of the course this upload belongs to
    pub belongs_to: Uuid, // TODO consider adding resolved values for faster API times

    /// The ID of the prof that held the course this upload belongs to
    pub held_by: Option<Uuid>, // TODO consider adding resolved values for faster API times
}
