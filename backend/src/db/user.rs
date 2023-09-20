use std::{str::FromStr, sync::Arc};

use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use email_address::EmailAddress;
use justerror::Error;
use rand::rngs::OsRng;
use sqlx::{self, Acquire, Pool, Postgres};
use uuid::Uuid;

use crate::{
    data::{User, UserWithEmails},
    db::SelectExistsTmp,
};

use super::SelectExists;

#[Error]
pub enum UserError {
    EmailInvalid(Arc<str>),
    EmailTaken(Arc<str>), // Zero-copy string; gotta go fast
    QueryError(sqlx::Error),
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
pub async fn register(db_pool: &Pool<Postgres>, user: UserWithEmails) -> Result<(), UserError> {
    let UserWithEmails {
        id,
        first_names,
        last_name,
        password_hash,
        totp_secret,
        emails,
        user_role,
    } = user;

    let mut tx = db_pool.begin().await.map_err(UserError::QueryError)?;

    // TODO make this parallel
    for email in emails.iter() {
        if !EmailAddress::is_valid(email) {
            return Err(UserError::EmailInvalid(Arc::from(email.as_str())));
        }

        let db_con = tx.acquire().await.map_err(UserError::QueryError)?;

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
        .map(|tmp| SelectExists::from(tmp).0)?;

        if email_taken {
            return Err(UserError::EmailTaken(Arc::from(email.as_str())));
        }
    }

    let db_con = tx.acquire().await.map_err(UserError::QueryError)?;

    let mail_uuid = Uuid::new_v4();
    let email_address = EmailAddress::from_str(&emails[0]).unwrap(); // We validated this earlier

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
            INSERT INTO "user" (id, first_names, last_name, primary_email, password_hash, totp_secret, user_role)
            VALUES ($5, $6, $7, $8, $9, $10, $11)
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
        mail_uuid,
        &*password_hash,
        totp_secret.as_deref(),
        user_role,
    )
    .execute(db_con)
    .await
    .map_err(UserError::QueryError)?;

    tx.commit().await.map_err(UserError::QueryError)?;

    Ok(())
}

pub async fn get_user_by_email(db_pool: &Pool<Postgres>, email: &str) -> Option<User> {
    let user = sqlx::query!(
        r#"
            SELECT u.id, first_names, last_name, password_hash, totp_secret, user_role
            FROM "user" AS u
            INNER JOIN email ON primary_email = email.id
            WHERE email.address = $1
        "#,
        email
    )
    .fetch_optional(db_pool)
    .await
    .expect("Failed to query user")
    .map(|user| User {
        id: user.id,
        first_names: user.first_names.into(),
        last_name: user.last_name.into(),
        password_hash: user.password_hash.into(),
        totp_secret: user.totp_secret,
        user_role: user.user_role,
    });

    user
}

pub fn make_pwd_hash(pwd: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = &Argon2::default();

    let password_hash = argon2
        .hash_password(&pwd.as_bytes(), &salt) // Allocates twice (once for the `String`)
        .unwrap()
        .serialize()
        .as_str()
        .into();

    password_hash
}
