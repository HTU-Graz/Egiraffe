use anyhow::Context;
use sqlx::{PgPool, PgTransaction};

use crate::data::Course;

pub async fn create_course(tx: &mut PgTransaction<'_>, course: &Course) -> anyhow::Result<()> {
    sqlx::query!(
        "
        INSERT INTO
            courses (id, held_at, course_name)
        VALUES
            ($1, $2, $3)
        ",
        course.id,
        course.held_at,
        course.name,
    )
    .execute(&mut **tx)
    .await
    .context("Failed to create course")?;

    Ok(())
}

/// Finds the course with the given ID using a `WHERE` clause and replaces it with the given course
/// keeping the ID.
pub async fn replace_course(db_pool: &PgPool, course: Course) -> anyhow::Result<()> {
    sqlx::query!(
        "
        UPDATE
            courses
        SET
            held_at = $2,
            course_name = $3
        WHERE
            id = $1
        ",
        course.id,
        course.held_at,
        course.name,
    )
    .execute(db_pool)
    .await
    .context("Failed to replace course")?;

    Ok(())
}

pub async fn get_courses(db_pool: &PgPool) -> anyhow::Result<Vec<Course>> {
    sqlx::query_as!(
        Course,
        "
        SELECT
            id,
            held_at,
            course_name AS name
        FROM
            courses
        ",
    )
    .fetch_all(db_pool)
    .await
    .context("Failed to get courses")
}
