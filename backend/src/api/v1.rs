pub mod action;
mod auth;
mod content;
mod course;
mod ecs;
mod get;
mod profs;
// mod university;

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, put},
    Json, Router,
};
use axum_extra::extract::CookieJar;
use serde_json::json;
use uuid::Uuid;

use crate::{
    db::{self, session::ValidationResult},
    AppState,
};

use super::api_greeting;

const SESSION_COOKIE_NAME: &str = "egiraffe_session_token";

pub fn routes(state: &AppState) -> Router<AppState> {
    use AuthLevel::*;

    Router::new()
        .route("/", get(api_greeting).post(api_greeting).put(api_greeting))
        .nest(
            "/auth",
            Router::new()
                .route("/login", put(auth::handle_login))
                .route("/register", put(auth::handle_register))
                .route("/logout", put(auth::handle_logout)),
        )
        .nest(
            "/get",
            get::routes(state).layer(middleware::from_fn_with_state(
                state.clone(),
                auth::<Anyone>,
            )),
        )
        .nest(
            "/do",
            action::routes(state).layer(middleware::from_fn_with_state(
                state.clone(),
                auth::<RegularUser>,
            )),
        )
        .nest(
            "/mod",
            Router::new()
                .route("/demo-mod-route", get(handle_demo_protected_route))
                .nest("/courses", course::routes(state))
                .nest("/profs", profs::routes(state))
                .nest("/content", content::routes(state))
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    auth::<Moderator>,
                )),
        )
        .nest(
            "/admin",
            Router::new()
                .route("/demo-admin-route", get(handle_demo_protected_route))
                .nest("/ecs", ecs::routes(state))
                // .nest("/university", university::routes(state))
                .layer(middleware::from_fn_with_state(state.clone(), auth::<Admin>)),
        )
        .route(
            "/demo-protected-route",
            get(handle_demo_protected_route).layer(middleware::from_fn_with_state(
                state.clone(),
                auth::<RegularUser>,
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

async fn auth<const REQUIRED_AUTH_LEVEL: i16>(
    State(db_pool): State<AppState>,
    cookie_jar: CookieJar,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    assert!(
        REQUIRED_AUTH_LEVEL >= AuthLevel::Anyone && REQUIRED_AUTH_LEVEL <= AuthLevel::Admin,
        "Invalid auth level"
    );

    log::info!("Auth level required: {}", REQUIRED_AUTH_LEVEL);

    let unauthorized = (
        StatusCode::UNAUTHORIZED,
        Json(json!({ "success": false, "message": "Unauthorized" })),
    );

    let Some(session_cookie) = cookie_jar.get(SESSION_COOKIE_NAME) else {
        log::info!("No session cookie");

        if REQUIRED_AUTH_LEVEL == AuthLevel::Anyone {
            request.extensions_mut().insert(Uuid::nil());
            return Ok(next.run(request).await);
        } else {
            return Err(unauthorized);
        }
    };

    match db::session::validate_session(&db_pool, &session_cookie.value().to_string()).await {
        ValidationResult::Valid {
            auth_level,
            user_id,
        } if auth_level >= REQUIRED_AUTH_LEVEL => {
            request.extensions_mut().insert(user_id);
            let response = next.run(request).await;
            Ok(response)
        }
        ValidationResult::Invalid if REQUIRED_AUTH_LEVEL == AuthLevel::Anyone => {
            request.extensions_mut().insert(Uuid::nil());
            return Ok(next.run(request).await);
        }
        ValidationResult::Invalid | ValidationResult::Valid { .. } => {
            log::info!("Invalid session");
            Err(unauthorized)
        }
    }
}
