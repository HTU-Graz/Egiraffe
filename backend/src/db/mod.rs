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
}

#[derive(sqlx::FromRow)]
struct SelectExists {
    exists: bool,
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

    let primary_email = &emails[0];

    let email_taken = sqlx::query_as!(
        bool,
        r#"
            SELECT EXISTS (
                SELECT 1
                FROM email
                WHERE address = $1
            )
        "#,
        &[*primary_email]
    );

    Ok(())
}
