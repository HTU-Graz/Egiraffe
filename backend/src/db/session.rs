use base64::{engine::general_purpose, Engine as _};
use sqlx::{Acquire, Pool, Postgres};
use uuid::Uuid;

use crate::data::Token;

pub enum ValidationResult {
    Valid { user_id: Uuid },
    Invalid,
}

pub async fn validate_session(db_pool: &Pool<Postgres>, token: &Token) -> ValidationResult {
    let mut tx = db_pool
        .begin()
        .await
        // .map_err(UserError::QueryError)
        // ?
        .unwrap();

    let db_con = tx
        .acquire()
        .await
        // .map_err(UserError::QueryError)
        // ?
        .unwrap();

    let token: String = general_purpose::STANDARD_NO_PAD.encode(token);

    let session = sqlx::query!(
        r#"
            SELECT of_user
            FROM session
            WHERE token = $1
        "#,
        token
    )
    .fetch_optional(db_con)
    .await
    .expect("Failed to query session")
    .map(|session| session.of_user);

    drop(tx);

    match session {
        Some(Some(user_id)) => ValidationResult::Valid { user_id },
        _ => ValidationResult::Invalid,
    }
}

// pub async fn create_session(db_pool: &Pool<Postgres>, user_id: Uuid) -> Token {
pub async fn create_session(db_pool: &Pool<Postgres>, user_id: Uuid) -> String {
    let mut tx = db_pool
        .begin()
        .await
        // .map_err(UserError::QueryError)
        // ?
        .unwrap();

    let db_con = tx
        .acquire()
        .await
        // .map_err(UserError::QueryError)
        // ?
        .unwrap();

    // 32 bytes of random data
    let token: Token = rand::random();

    let token: String = general_purpose::URL_SAFE_NO_PAD.encode(&token);

    // TODO comment this back in once `api::v1::handle_login` is implemented
    // sqlx::query!(
    //     r#"
    //             INSERT INTO session (id, token, of_user)
    //             VALUES ($1, $2, $3)
    //         "#,
    //     Uuid::new_v4(),
    //     token,
    //     user_id
    // )
    // .execute(db_con)
    // .await
    // .expect("Failed to create session");

    drop(tx);

    token
}
