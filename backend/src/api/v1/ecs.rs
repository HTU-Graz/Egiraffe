use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::{api::api_greeting, data::Prof, db, AppState};

pub fn routes(state: &AppState) -> Router<AppState> {
    Router::new().route("/", get(api_greeting).post(api_greeting).put(api_greeting))
    // .route("/get-user-balance", put(handle_get_user_balance))
    // .route(
    //     "/create-system-transaction",
    // put(handle_create_system_transaction),
    // )
}

// pub async fn handle_get_user_balance(
//     Json(user_id): Json<Uuid>,
//     State(state): State<AppState>,
// ) -> impl IntoResponse {
//     (
//         StatusCode::INTERNAL_SERVER_ERROR,
//         json!({ "error": "Not implemented" }),
//     )

//     // let balance = db:ecs::get_user_balance(&state.db, user_id).await;
//     // match balance {
//     //     Ok(balance) => (StatusCode::OK, json!({ "balance": balance })),
//     //     Err(e) => (
//     //         StatusCode::INTERNAL_SERVER_ERROR,
//     //         json!({ "error": e.to_string() }),
//     //     ),
//     // }
// }

// pub async fn handle_create_system_transaction(
//     Json((user_id, amount, reason)): Json<(Uuid, i64, String)>,
//     State(state): State<AppState>,
// ) -> impl IntoResponse {
//     (
//         StatusCode::INTERNAL_SERVER_ERROR,
//         json!({ "error": "Not implemented" }),
//     )

//     // let result = db::ecs::create_system_transaction(&state.db, user_id, amount, &reason).await;
//     // match result {
//     //     Ok(_) => (StatusCode::OK, json!({})),
//     //     Err(e) => (
//     //         StatusCode::INTERNAL_SERVER_ERROR,
//     //         json!({ "error": e.to_string() }),
//     //     ),
//     // }
// }
