use axum::{
    response::IntoResponse,
    routing::{get, put},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::Router;

use super::api_greeting;

pub fn routes() -> Router {
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
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct RegisterRes {
    pub success: bool,
    pub email: String,
}

async fn handle_register(Json(register_data): Json<RegisterReq>) -> impl IntoResponse {
    log::info!("Register attempt for email {}", register_data.email);

    Json(RegisterRes {
        success: true,
        email: register_data.email,
    })
}

#[derive(Serialize, Debug)]
pub struct LogoutRes {
    pub success: bool,
}

async fn handle_logout() -> impl IntoResponse {
    Json(LogoutRes { success: true })
}
