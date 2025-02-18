use std::sync::Arc;

use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use time::Duration;
use uuid::Uuid;

use crate::{
    api::v1::{AuthLevel, SESSION_COOKIE_NAME},
    data::UserWithEmails,
    db::{self, DB_POOL},
    mail::{send_activation_mail},
};

#[derive(Deserialize, Debug)]
pub struct LoginReq {
    pub email: String,
    pub password: String,
    pub totp: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct LoginRes {
    pub success: bool,
    pub email: String,
}

#[derive(Deserialize, Debug)]
pub struct RegisterReq {
    pub first_names: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct ActivationReq {
    pub token: String
}

#[derive(Serialize, Debug)]
pub struct RegisterRes {
    pub success: bool,
    // pub email: String,
}

#[derive(Serialize, Debug)]
pub struct LogoutRes {
    pub success: bool,
}

pub async fn handle_login(
    cookie_jar: CookieJar,
    Json(login_data): Json<LoginReq>,
) -> impl IntoResponse {
    let db_pool = *DB_POOL.get().unwrap();

    let Some(user) = db::user::get_active_user_by_email(&db_pool, &login_data.email).await else {
        log::info!("Login failed: no user with email {}", login_data.email);
        return (
            StatusCode::BAD_REQUEST,
            cookie_jar,
            Json(LoginRes {
                success: false,
                email: login_data.email,
            }),
        );
    };

    // FIXME timing side channel

    let argon2 = Argon2::default();
    let password_hash = PasswordHash::new(&user.password_hash).unwrap();
    let password_valid = argon2.verify_password(login_data.password.as_bytes(), &password_hash);

    if password_valid.is_err() {
        log::info!(
            "Login failed: wrong password for email {}",
            login_data.email
        );
        return (
            StatusCode::BAD_REQUEST,
            cookie_jar,
            Json(LoginRes {
                success: false,
                email: login_data.email,
            }),
        );
    }

    let token = db::session::create_session(&db_pool, user.id).await;

    log::info!("Login successful for email {}", login_data.email);

    let mut session_cookie = Cookie::new(SESSION_COOKIE_NAME, token);
    session_cookie.make_permanent();
    session_cookie.set_http_only(true);
    session_cookie.set_path("/");

    (
        StatusCode::OK,
        cookie_jar.add(session_cookie), // Good that this forces a new session in case a session already exists :) Otherwise we would have a session fixation vulnerability.
        Json(LoginRes {
            success: true,
            email: login_data.email,
        }),
    )
}

pub async fn handle_register(Json(register_data): Json<RegisterReq>) -> impl IntoResponse {
    let db_pool = *DB_POOL.get().unwrap();

    log::info!("Register attempt for email {}", register_data.email);

    let RegisterReq {
        first_names,
        last_name,
        email, // TODO verify email
        password,
    } = register_data;

    //TODO: Systematically validate all fields, e.g.:
    // * reasonable size (no DoS)
    // * that they don't contain possibly malicious characters

    // HACK the call to `unwrap` must be replaced with an error handling mechanism (return a 500 error)
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = &Argon2::default();
    let password_hash: Arc<str> = argon2
        .hash_password(password.as_bytes(), &salt) // Allocates twice (once for the `String`)
        .unwrap()
        .serialize()
        .as_str()
        .into(); // Allocates once (for the `Arc`) // TODO attempt to avoid this allocation

    // TODO reconsider if `Arc` is the right choice here
    //  does it perform an atomic reference count increment on creation?
    let user = UserWithEmails {
        id: Uuid::new_v4(),
        first_names: first_names.into(),
        last_name: last_name.into(),
        password_hash,
        totp_secret: None,
        emails: Arc::new(vec![email]),
        user_role: AuthLevel::RegularUser,
    };

    // TODO: I didn't manage yet to get the register()-function to work only with a reference
    let registration_result = db::user::register(&db_pool, user.clone()).await;

    match registration_result {
        Ok(_) => {
            log::info!("Registration successful");
        }
        Err(e) => {
            log::error!("Registration failed: {:?}", e);

            return (
                StatusCode::BAD_REQUEST,
                Json(RegisterRes { success: false }),
            );
        }
    }

//TODO
//    let mail_result = send_activation_mail(&user.first_names, &user.last_name, &user.emails[0], "TODO: Real token").await;

    (StatusCode::OK, Json(RegisterRes { success: true }))

}
pub async fn handle_activate(Json(activation_data): Json<ActivationReq>) -> impl IntoResponse {
    //TODO
    return (StatusCode::OK, Json(RegisterRes { success: true }));
}

pub async fn handle_logout(cookie_jar: CookieJar) -> impl IntoResponse {
    let db_pool = *DB_POOL.get().unwrap();

    let Some(cookie) = cookie_jar.get(SESSION_COOKIE_NAME) else {
        log::info!("Logout failed: no session cookie");
        return (
            StatusCode::OK, // We consider logouts to be idempotent
            cookie_jar,
            Json(LogoutRes { success: true }), // Thus we return success
        );
    };

    let db_session_deletion_status = db::session::delete_session(&db_pool, cookie.value()).await;

    if db_session_deletion_status.is_err() {
        log::error!("Failed to delete session from database");
    }

    log::info!("Logout");

    let dead_cookie = make_dead_cookie();

    (
        StatusCode::OK,
        cookie_jar.add(dead_cookie),
        Json(LogoutRes { success: true }),
    )
}

// TODO consider turning this into a static constant to clone from
pub fn make_dead_cookie() -> Cookie<'static> {
    let mut dead_cookie = Cookie::named(SESSION_COOKIE_NAME);
    dead_cookie.set_value("");
    dead_cookie.set_http_only(true);
    dead_cookie.set_path("/");
    dead_cookie.set_max_age(Some(Duration::ZERO));
    dead_cookie
}
