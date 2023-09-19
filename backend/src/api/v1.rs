use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
    Json, Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{data::UserWithEmails, db, AppState};

use super::api_greeting;

const SESSION_COOKIE_NAME: &str = "session_token";

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(api_greeting).post(api_greeting).put(api_greeting))
        .nest(
            "/auth",
            Router::new()
                .route("/login", put(handle_login))
                .route("/register", put(handle_register))
                .route("/logout", put(handle_logout)),
        )
}

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

async fn handle_login(
    State(db_pool): State<AppState>,
    cookie_jar: CookieJar,
    Json(login_data): Json<LoginReq>,
) -> impl IntoResponse {
    let user = match db::user::get_user_by_email(&db_pool, &login_data.email).await {
        Some(user) => user,
        None => {
            log::info!("Login failed: no user with email {}", login_data.email);
            return (
                StatusCode::BAD_REQUEST,
                cookie_jar,
                Json(LoginRes {
                    success: false,
                    email: login_data.email,
                }),
            );
        }
    };

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

    (
        StatusCode::OK,
        cookie_jar.add(session_cookie),
        Json(LoginRes {
            success: true,
            email: login_data.email,
        }),
    )
}

#[derive(Deserialize, Debug)]
pub struct RegisterReq {
    pub first_names: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct RegisterRes {
    pub success: bool,
    // pub email: String,
}

async fn handle_register(
    State(db_pool): State<AppState>,
    Json(register_data): Json<RegisterReq>,
) -> impl IntoResponse {
    log::info!("Register attempt for email {}", register_data.email);

    let RegisterReq {
        first_names,
        last_name,
        email,
        password,
    } = register_data;

    // HACK the call to `unwrap` must be replaced with an error handling mechanism (return a 500 error)
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = &Argon2::default();
    let password_hash: Arc<str> = argon2
        .hash_password(&password.as_bytes(), &salt) // Allocates twice (once for the `String`)
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
    };

    // TODO handle errors (see https://docs.rs/axum/latest/axum/error_handling/index.html#axums-error-handling-model)
    let registration_result = db::user::register(&db_pool, user).await;

    match registration_result {
        Ok(_) => {
            log::info!("Registration successful");
            (StatusCode::OK, Json(RegisterRes { success: true }))
        }
        Err(e) => {
            log::error!("Registration failed: {:?}", e);

            (
                StatusCode::BAD_REQUEST,
                Json(RegisterRes { success: false }),
            )
        }
    }
}

#[derive(Serialize, Debug)]
pub struct LogoutRes {
    pub success: bool,
}

async fn handle_logout(
    State(_db_pool): State<AppState>,
    cookie_jar: CookieJar,
) -> impl IntoResponse {
    // TODO remove the session from the database

    log::info!("Logout");

    (
        cookie_jar.remove(Cookie::named(SESSION_COOKIE_NAME)),
        Json(LogoutRes { success: true }),
    )
}
