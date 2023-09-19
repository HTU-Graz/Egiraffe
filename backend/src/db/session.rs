use anyhow::Context;
use base64::{engine::general_purpose, Engine as _};
use sqlx::{Acquire, Pool, Postgres};
use uuid::Uuid;

use crate::data::Token;

pub enum ValidationResult {
    Valid { user_id: Uuid, auth_level: i16 },
    Invalid,
}

// pub async fn validate_session(db_pool: &Pool<Postgres>, token: &Token) -> ValidationResult {
//     let token: String = general_purpose::STANDARD_NO_PAD.encode(token);
pub async fn validate_session(db_pool: &Pool<Postgres>, token: &String) -> ValidationResult {
    // let token: String = general_purpose::STANDARD_NO_PAD.encode(token);

    log::info!("Validating session with token {}", token);

    let session = sqlx::query!(
        r#"
            SELECT s.of_user, u.user_role AS "auth_level"
            FROM session AS s
            INNER JOIN "user" AS u
                ON s.of_user = u.id
            WHERE token = $1
        "#,
        token
    )
    .fetch_optional(db_pool)
    .await
    .expect("Failed to query session")
    .map(|session| (session.of_user, session.auth_level));

    match session {
        Some((Some(user_id), auth_level)) => {
            log::info!("Session is valid for user {}", user_id);
            ValidationResult::Valid {
                user_id,
                auth_level,
            }
        }
        _ => {
            log::info!("Session is invalid");
            ValidationResult::Invalid
        }
    }
}

// pub async fn create_session(db_pool: &Pool<Postgres>, user_id: Uuid) -> Token {
pub async fn create_session(db_pool: &Pool<Postgres>, user_id: Uuid) -> String {
    // 32 bytes of random data
    let token: Token = rand::random();

    let token: String = general_purpose::URL_SAFE_NO_PAD.encode(&token);

    sqlx::query!(
        r#"
            INSERT INTO session (id, token, of_user)
            VALUES ($1, $2, $3)
        "#,
        Uuid::new_v4(),
        token,
        user_id
    )
    .execute(db_pool)
    .await
    .expect("Failed to create session");

    token
}

pub async fn delete_session(db_pool: &Pool<Postgres>, value: &str) -> anyhow::Result<()> {
    log::info!("Deleting a session");

    sqlx::query!(
        r#"
            DELETE FROM session
            WHERE token = $1
        "#,
        value
    )
    .execute(db_pool)
    .await
    .context("Failed to delete session")?;

    Ok(())
}
