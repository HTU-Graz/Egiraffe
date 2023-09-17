use std::{str::FromStr, sync::Arc};

use email_address::EmailAddress;
use justerror::Error;
use sqlx::{postgres::PgPoolOptions, Acquire, Pool, Postgres};
use uuid::Uuid;

use crate::data::{University, User};

pub async fn connect() -> anyhow::Result<Pool<Postgres>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres@127.0.0.1/egiraffe")
        .await?;

    Ok(pool)
}

// TODO remove this
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

    let mut tx = db_pool
        .begin()
        .await
        .map_err(UserError::QueryError)
        // ?
        .unwrap();

    // TODO make this parallel
    for email in emails.iter() {
        if !EmailAddress::is_valid(email) {
            return Err(UserError::EmailInvalid(Arc::from(email.as_str())));
        }

        let db_con = tx
            .acquire()
            .await
            .map_err(UserError::QueryError)
            // ?
            .unwrap();

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
        .fetch_one(db_con)
        .await
        .map_err(UserError::QueryError)
        .map(|tmp| SelectExists::from(tmp).0)
        // ?;
        .unwrap();

        if email_taken {
            return Err(UserError::EmailTaken(Arc::from(email.as_str())));
        }
    }

    let db_con = tx
        .acquire()
        .await
        .map_err(UserError::QueryError)
        // ?
        .unwrap();

    let mail_uuid = Uuid::new_v4();
    let email_address = EmailAddress::from_str(&emails[0]).unwrap();

    sqlx::query!(
        r#"
            WITH matching_university AS (
                SELECT id
                FROM university
                WHERE $1 = ANY (domain_names)
            ),
            new_email AS (
                INSERT INTO email (id, address, belongs_to_user, of_university, status)
                VALUES ($2, $3, $4, (SELECT id FROM matching_university), 'unverified')
            )
            INSERT INTO "user" (id, first_names, last_name, password_hash, totp_secret)
            VALUES ($5, $6, $7, $8, $9)
        "#,
        // University
        email_address.domain(),
        // Email
        mail_uuid,
        &*emails[0],
        id,
        // User
        id,
        &*first_names,
        &*last_name,
        &*password_hash,
        totp_secret.as_deref(),
    )
    .execute(db_con)
    .await
    .map_err(UserError::QueryError)
    // ?;
    .unwrap();

    tx.commit()
        .await
        .map_err(UserError::QueryError)
        // ?
        .unwrap();

    Ok(())
}

/*
CREATE TABLE IF NOT EXISTS public.university
(
    id uuid,
    name_full character varying(100) NOT NULL,
    name_mid character varying(50) NOT NULL,
    name_short character varying(50) NOT NULL,
    domain_names character varying(100)[] NOT NULL,
    PRIMARY KEY (id)
);

*/

pub async fn create_universities(db_pool: &Pool<Postgres>) -> anyhow::Result<()> {
    let mut tx = db_pool.begin().await?;

    let db_con = tx.acquire().await?;

    let unis = [
        University {
            id: Uuid::new_v4(),
            full_name: "Technische Universität Graz",
            mid_name: "TU Graz",
            short_name: "TUG",
            domain_names: &["tugraz.at".to_string(), "student.tugraz.at".to_string()],
        },
        University {
            id: Uuid::new_v4(),
            full_name: "Karl Franzens Universität Graz",
            mid_name: "Uni Graz",
            short_name: "KFU",
            domain_names: &["uni-graz.at".to_string()],
        },
    ];

    for uni in unis {
        let University {
            id,
            full_name,
            mid_name,
            short_name,
            domain_names,
        } = uni;

        sqlx::query!(
            r#"
            INSERT INTO university (id, name_full, name_mid, name_short, domain_names)
            VALUES ($1, $2, $3, $4, $5)
        "#,
            id,
            full_name,
            mid_name,
            short_name,
            &domain_names
        )
        .execute(&mut *db_con)
        .await?;
    }

    tx.commit().await?;

    Ok(())
}
