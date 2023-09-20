mod auth;
mod course;
mod get;

use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, put},
    Json, Router,
};
use axum_extra::extract::CookieJar;
use serde_json::json;

use crate::{
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
                .route("/login", put(auth::handle_login))
                .route("/register", put(auth::handle_register))
                .route("/logout", put(auth::handle_logout)),
        )
        .nest("/get", get::routes(state))
        .nest(
            "/mod",
            Router::new()
                .route("/demo-mod-route", get(handle_demo_protected_route))
                .nest("/courses", course::routes(state))
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
    assert!(
        REQUIRED_AUTH_LEVEL > AuthLevel::Anyone && REQUIRED_AUTH_LEVEL <= AuthLevel::Admin,
        "Invalid auth level"
    );

    log::info!("Auth level required: {}", REQUIRED_AUTH_LEVEL);

    if REQUIRED_AUTH_LEVEL == AuthLevel::Anyone {
        return Ok(next.run(request).await);
    }

    let unauthorized = (
        StatusCode::UNAUTHORIZED,
        Json(json!({ "error": "Unauthorized" })),
    );

    let Some(session_cookie) = cookie_jar.get(SESSION_COOKIE_NAME) else {
        log::info!("No session cookie");
        return Err(unauthorized);
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
