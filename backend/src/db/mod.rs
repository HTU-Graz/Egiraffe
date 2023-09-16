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
    EmailTaken,
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

pub async fn register_user(db_pool: &Pool<Postgres>, user: User) -> Result<(), UserError> {
    let User {
        id,
        first_names,
        last_name,
        password_hash,
        totp_secret,
        emails,
    } = user;

    for email in &*emails {
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
        .map_err(UserError::QueryError)?;

        // TODO I'd rather have this be an `.into()` call after `map_err` but I can't figure out how to do that right now
        if SelectExists::from(email_taken).0 {
            return Err(UserError::EmailTaken);
        }
    }

    Ok(())
}
