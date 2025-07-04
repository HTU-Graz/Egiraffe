use std::{str::FromStr, sync::Arc};

use anyhow::Context;
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use email_address::EmailAddress;
use justerror::Error;
use rand::rngs::OsRng;
use sqlx::{self, Acquire, PgTransaction, Pool, Postgres};
use uuid::Uuid;

use crate::{
    api::v1::action::DoMeReq,
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
pub async fn register(
    mut tx: &mut PgTransaction<'_>,
    user: UserWithEmails,
) -> Result<(), UserError> {
    let UserWithEmails {
        id,
        first_names,
        last_name,
        password_hash,
        totp_secret,
        emails,
        user_role,
        nick,
    } = user;

    // TODO make this parallel
    for email in emails.iter() {
        if !EmailAddress::is_valid(email) {
            return Err(UserError::EmailInvalid(Arc::from(email.as_str())));
        }

        let email_taken = sqlx::query_as!(
            SelectExistsTmp,
            "
            SELECT
                EXISTS (
                    SELECT
                        1
                    FROM
                        emails
                    WHERE
                        address = $1
                )
            ",
            email
        )
        .fetch_one(&mut **tx)
        .await
        .map_err(UserError::QueryError)
        .map(|tmp| SelectExists::from(tmp).0)?;

        if email_taken {
            return Err(UserError::EmailTaken(Arc::from(email.as_str())));
        }
    }

    let mail_uuid = Uuid::new_v4();
    let email_address = EmailAddress::from_str(&emails[0]).unwrap(); // We validated this earlier

    // HACK the email domain verification has been disabled for now
    sqlx::query!(
        "
        WITH matching_university AS (
            SELECT
                id
            FROM
                universities
             WHERE
                $1 = ANY (email_domain_names)
                OR true
            LIMIT 1
        ),
        new_email AS (
            INSERT INTO
                emails (
                    id,
                    address,
                    belongs_to_user,
                    of_university,
                    STATUS
                )
            VALUES
                (
                    $2,
                    $3,
                    $4,
                    (
                        SELECT
                            id
                        FROM
                            matching_university
                    ),
                    'unverified'
                )
        )
        INSERT INTO
            users (
                id,
                first_names,
                last_name,
                primary_email,
                password_hash,
                totp_secret,
                user_role,
                nick
            )
        VALUES
            ($5, $6, $7, $8, $9, $10, $11, $12)
        ",
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
        nick,
    )
    .execute(&mut **tx)
    .await
    .map_err(UserError::QueryError)?;

    Ok(())
}

pub async fn get_active_user_by_email(mut tx: &mut PgTransaction<'_>, email: &str) -> Option<User> {
    //TODO: Do we only allow login by primary Mail? Don't we want this:
    //INNER JOIN email ON u.id = email.belongs_to_user
    // - also for the other functions
    sqlx::query!(
        r#"
        SELECT
            u.id,
            first_names,
            last_name,
            password_hash,
            totp_secret,
            user_role
        FROM
            users AS u
            INNER JOIN emails ON primary_email = emails.id
        WHERE
            emails.address = $1
        "#,
        email
    )
    .fetch_optional(&mut **tx)
    .await
    .expect("Failed to query user")
    .map(|user| User {
        id: user.id,
        first_names: user.first_names,
        last_name: user.last_name,
        password_hash: user.password_hash,
        totp_secret: user.totp_secret,
        user_role: user.user_role,
    })
}

pub fn make_pwd_hash(pwd: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = &Argon2::default();

    let password_hash = argon2
        .hash_password(pwd.as_bytes(), &salt) // Allocates twice (once for the `String`)
        .unwrap()
        .serialize()
        .as_str()
        .into();

    password_hash
}

pub async fn get_user_by_session(
    mut tx: &mut PgTransaction<'_>,
    session_cookie: &str,
) -> anyhow::Result<User> {
    let user = sqlx::query!(
        "
        SELECT
            u.id,
            first_names,
            last_name,
            password_hash,
            totp_secret,
            user_role
        FROM
            users AS u
            INNER JOIN sessions ON u.id = sessions.of_user
        WHERE
            sessions.token = $1
        ",
        session_cookie
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(User {
        id: user.id,
        first_names: user.first_names,
        last_name: user.last_name,
        password_hash: user.password_hash,
        totp_secret: user.totp_secret,
        user_role: user.user_role,
    })
}

// TODO remove this?
// /// Update first_names, last_name, and password (gets hashed) for a user
// pub async fn update_user_safe(
//     mut tx: &mut PgTransaction<'_>,
//     user: DoMeReq,
// ) -> anyhow::Result<UserWithEmails> {
//     let DoMeReq {
//         first_names,
//         last_name,
//         password,
//     } = user;
//     Ok(user)
// }

pub async fn get_user_by_id(
    mut tx: &mut PgTransaction<'_>,
    current_user_id: Uuid,
) -> anyhow::Result<Option<UserWithEmails>> {
    let user = sqlx::query!(
        "
        SELECT
            u.id,
            first_names,
            last_name,
            password_hash,
            totp_secret,
            user_role,
            emails.address AS emails,
            nick
        FROM
            users AS u
            INNER JOIN emails ON primary_email = emails.id
        WHERE
            u.id = $1
        ",
        current_user_id
    )
    .fetch_one(&mut **tx)
    .await
    .map(|user| UserWithEmails {
        id: user.id,
        first_names: user.first_names.expect("User has no first name").into(),
        last_name: user.last_name.expect("User has no last name").into(),
        password_hash: user.password_hash.into(),
        totp_secret: user.totp_secret.map(|s| s.into()),
        // emails: user.emails.expect("User has no emails"),
        emails: Arc::new(vec![user.emails.expect("User has no emails")]), // TODO check if this is correct
        user_role: user.user_role,
        nick: user.nick,
    });

    Ok(Some(user?))
}

/// Update an existing user in the database.
pub async fn update_user(
    mut tx: &mut PgTransaction<'_>,
    user: UserWithEmails,
) -> anyhow::Result<()> {
    let UserWithEmails {
        id,
        first_names,
        last_name,
        password_hash,
        totp_secret,
        // emails,
        user_role,
        ..
    } = user;

    // TODO handle email updates

    let db_con = tx.acquire().await?;

    sqlx::query!(
        "
        UPDATE
            users
        SET
            first_names = $1,
            last_name = $2,
            password_hash = $3,
            totp_secret = $4,
            user_role = $5
        WHERE
            id = $6
        ",
        &*first_names,
        &*last_name,
        &*password_hash,
        totp_secret.as_deref(),
        user_role,
        id,
    )
    .execute(db_con)
    .await?;

    Ok(())
}
