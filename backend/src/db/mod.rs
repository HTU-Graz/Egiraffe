use std::sync::Arc;

use email_address::EmailAddress;
use justerror::Error;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::data::User;

pub async fn connect() -> anyhow::Result<Pool<Postgres>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres@127.0.0.1/egiraffe")
        .await?;

    Ok(pool)
}

pub(crate) async fn demo(db_pool: &Pool<Postgres>) -> anyhow::Result<()> {
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(42_i64)
        .fetch_one(db_pool)
        .await?;

    assert_eq!(row.0, 42);

    Ok(())
}

#[Error]
pub enum UserError {
    EmailInvalid(Arc<str>),
    EmailTaken(Arc<str>), // Zero-copy string; gotta go fast
    QueryError(sqlx::Error),
}

#[derive(sqlx::FromRow)]
struct SelectExistsTmp {
    exists: Option<bool>,
}

struct SelectExists(bool);

impl From<SelectExistsTmp> for SelectExists {
    fn from(tmp: SelectExistsTmp) -> Self {
        Self(tmp.exists.unwrap_or(false))
    }
}

/// Register a user in the database with checks for email validity and uniqueness.
///
/// # Errors
///
/// - [`UserError::EmailInvalid`] if the email is invalid
/// - [`UserError::EmailTaken`] if the email is already taken
/// - [`UserError::QueryError`] if the query fails (including the underlying database error)
///
/// # Panics
///
/// This function panics if the database connection pool is full.
pub async fn register_user(db_pool: &Pool<Postgres>, user: User) -> Result<(), UserError> {
    let User {
        id,
        first_names,
        last_name,
        password_hash,
        totp_secret,
        emails,
    } = user;

    // TODO make this parallel
    // HACK this is not properly executed in a transaction
    for email in emails.iter() {
        if !EmailAddress::is_valid(email) {
            return Err(UserError::EmailInvalid(Arc::from(email.as_str())));
        }

        let email_taken = sqlx::query_as!(
            SelectExistsTmp,
            r#"
                SELECT EXISTS (
                    SELECT 1
                    FROM email
                    WHERE address = $1
                )
        "#,
            email
        )
        .fetch_one(db_pool)
        .await
        .map_err(UserError::QueryError)
        .map(|tmp| SelectExists::from(tmp).0)?;

        if email_taken {
            return Err(UserError::EmailTaken(Arc::from(email.as_str())));
        }
    }

    sqlx::query!(
        r#"
            INSERT INTO "user" (id, first_names, last_name, password_hash, totp_secret)
            VALUES ($1, $2, $3, $4, $5)
        "#,
        id,
        &*first_names,
        &*last_name,
        &*password_hash,
        totp_secret.as_deref()
    )
    .execute(db_pool)
    .await
    .map_err(UserError::QueryError)?;

    Ok(())
}
