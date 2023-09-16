use std::sync::Arc;

use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{data::User, db, AppState};

use super::api_greeting;

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

async fn handle_login(Json(login_data): Json<LoginReq>) -> impl IntoResponse {
    log::info!("Login attempt for email {}", login_data.email);

    Json(LoginRes {
        success: true,
        email: login_data.email,
    })
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

    // TODO reconsider if `Arc` is the right choice here
    //  does it perform an atomic reference count increment on creation?
    let user = User {
        id: Uuid::new_v4(),
        first_names: first_names.into(),
        last_name: last_name.into(),
        password_hash: password.into(),
        totp_secret: None,
        emails: Arc::new(vec![email]),
    };

    // TODO handle errors (see https://docs.rs/axum/latest/axum/error_handling/index.html#axums-error-handling-model)
    db::register_user(&db_pool, user).await.unwrap();

    Json(RegisterRes {
        success: true,
        // email: user.emails[0].clone(),
    })
}

#[derive(Serialize, Debug)]
pub struct LogoutRes {
    pub success: bool,
}

async fn handle_logout() -> impl IntoResponse {
    Json(LogoutRes { success: true })
}
