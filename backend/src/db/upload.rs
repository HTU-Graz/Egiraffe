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
