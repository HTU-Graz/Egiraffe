use anyhow::Context;
use sqlx::PgPool;

use crate::data::Course;

pub async fn create_course(db_pool: &PgPool, course: Course) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
            INSERT INTO course (id, held_at, course_name)
            VALUES ($1, $2, $3)
        "#,
        course.id,
        course.held_at,
        course.name,
    )
    .execute(db_pool)
    .await
    .context("Failed to create course")?;

    Ok(())
}
