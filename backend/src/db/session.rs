use anyhow::Context;
use base64::{engine::general_purpose, Engine as _};
use sqlx::{PgTransaction, Pool, Postgres};
use uuid::Uuid;

use crate::data::Token;

pub enum ValidationResult {
    Valid { user_id: Uuid, auth_level: i16 },
    Invalid,
}

// pub async fn validate_session(mut tx: &mut PgTransaction, token: &Token) -> ValidationResult {
//     let token: String = general_purpose::STANDARD_NO_PAD.encode(token);
pub async fn validate_session(mut tx: &mut PgTransaction<'_>, token: &String) -> ValidationResult {
    log::info!("Validating session with a token");

    let session = sqlx::query!(
        "
        SELECT
            s.of_user,
            u.user_role AS auth_level
        FROM
            sessions AS s
            INNER JOIN users AS u ON s.of_user = u.id
        WHERE
            token = $1
        ",
        token
    )
    .fetch_optional(&mut **tx)
    .await
    .expect("Failed to query session")
    .map(|session| (session.of_user, session.auth_level));

    if let Some((Some(user_id), auth_level)) = session {
        log::info!("Session is valid for user {}", user_id);
        ValidationResult::Valid {
            user_id,
            auth_level,
        }
    } else {
        log::info!("Session is invalid");
        ValidationResult::Invalid
    }
}

// pub async fn create_session(mut tx: &mut PgTransaction, user_id: Uuid) -> Token {
pub async fn create_session(mut tx: &mut PgTransaction<'_>, user_id: Uuid) -> String {
    // 32 bytes of random data
    let token: Token = rand::random();

    let token: String = general_purpose::URL_SAFE_NO_PAD.encode(token);

    sqlx::query!(
        "
        INSERT INTO
            sessions (id, token, of_user)
        VALUES
            ($1, $2, $3)
        ",
        Uuid::new_v4(),
        token,
        user_id
    )
    .execute(&mut **tx)
    .await
    .expect("Failed to create session");

    token
}

pub async fn delete_session(mut tx: &mut PgTransaction<'_>, value: &str) -> anyhow::Result<()> {
    log::info!("Deleting a session");

    sqlx::query!(
        "
        DELETE FROM
            sessions
        WHERE
            token = $1
        ",
        value
    )
    .execute(&mut **tx)
    .await
    .context("Failed to delete session")?;

    Ok(())
}
