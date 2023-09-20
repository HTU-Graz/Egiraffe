use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, put},
    Json, Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use serde::{Deserialize, Serialize};
use serde_json::json;
use time::Duration;
use uuid::Uuid;

use crate::{
    data::UserWithEmails,
    db::{self, session::ValidationResult},
    AppState,
};

use super::api_greeting;

const SESSION_COOKIE_NAME: &str = "session_token";

pub fn routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(api_greeting).post(api_greeting).put(api_greeting))
        .nest(
            "/auth",
            Router::new()
                .route("/login", put(handle_login))
                .route("/register", put(handle_register))
                .route("/logout", put(handle_logout)),
        )
        .nest(
            "/mod",
            Router::new()
                .route("/demo-mod-route", get(handle_demo_protected_route))
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    auth::<_, { AuthLevel::Moderator }>,
                )),
        )
        .nest(
            "/admin",
            Router::new()
                .route("/demo-admin-route", get(handle_demo_protected_route))
                .layer(middleware::from_fn_with_state(
                    state.clone(),
                    auth::<_, { AuthLevel::Admin }>,
                )),
        )
        .route(
            "/demo-protected-route",
            get(handle_demo_protected_route).layer(middleware::from_fn_with_state(
                state.clone(),
                auth::<_, { AuthLevel::RegularUser }>,
            )),
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
    session_cookie.set_path("/");

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
        user_role: AuthLevel::RegularUser,
    };

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
    State(db_pool): State<AppState>,
    cookie_jar: CookieJar,
) -> impl IntoResponse {
    let cookie = match cookie_jar.get(SESSION_COOKIE_NAME) {
        Some(cookie) => cookie,
        None => {
            log::info!("Logout failed: no session cookie");
            return (
                StatusCode::OK, // We consider logouts to be idempotent
                cookie_jar,
                Json(LogoutRes { success: true }), // Thus we return success
            );
        }
    };

    let db_session_deletion_status = db::session::delete_session(&db_pool, &cookie.value()).await;

    if db_session_deletion_status.is_err() {
        log::error!("Failed to delete session from database");
    }

    log::info!("Logout");

    let mut dead_cookie = Cookie::named(SESSION_COOKIE_NAME);
    dead_cookie.set_value("");
    dead_cookie.set_http_only(true);
    dead_cookie.set_path("/");
    dead_cookie.set_max_age(Some(Duration::ZERO));

    (
        StatusCode::OK,
        cookie_jar.add(dead_cookie),
        Json(LogoutRes { success: true }),
    )
}

pub async fn handle_demo_protected_route() -> impl IntoResponse {
    log::info!("Demo protected route");

    (StatusCode::OK, "Hello, world!")
}

// Just pretend it's an enum, ok?
pub mod AuthLevel {
    #![allow(non_upper_case_globals, non_snake_case, dead_code)]
    pub const Anyone: i16 = 0;
    pub const RegularUser: i16 = 1;
    pub const Moderator: i16 = 2;
    pub const Admin: i16 = 3;
}

async fn auth<B, const REQUIRED_AUTH_LEVEL: i16>(
    State(db_pool): State<AppState>,
    cookie_jar: CookieJar,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    if REQUIRED_AUTH_LEVEL > AuthLevel::Admin {
        panic!("Invalid auth level");
    }

    log::info!("Auth level required: {}", REQUIRED_AUTH_LEVEL);

    if REQUIRED_AUTH_LEVEL == AuthLevel::Anyone {
        return Ok(next.run(request).await);
    }

    let unauthorized = (
        StatusCode::UNAUTHORIZED,
        Json(json!({ "error": "Unauthorized" })),
    );

    let session_cookie = match cookie_jar.get(SESSION_COOKIE_NAME) {
        Some(session_cookie) => session_cookie,
        None => {
            log::info!("No session cookie");
            return Err(unauthorized);
        }
    };

    match db::session::validate_session(&db_pool, &session_cookie.value().to_string()).await {
        ValidationResult::Valid { auth_level, .. } if auth_level >= REQUIRED_AUTH_LEVEL => {
            let response = next.run(request).await;
            Ok(response)
        }
        ValidationResult::Invalid | ValidationResult::Valid { .. } => {
            log::info!("Invalid session");
            Err(unauthorized)
        }
    }
}
