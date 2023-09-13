use std::net::{Ipv4Addr, SocketAddr};

use axum::{response::IntoResponse, routing::get, Router};
use tower_http::services::{ServeDir, ServeFile};

// Make sure to build the frontend first!
const STATIC_DIR: &str = "../frontend/dist";
const INDEX_FILE: &str = "../frontend/dist/index.html";

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let app = Router::new()
        .route(
            "/api",
            get(placeholder_api)
                .post(placeholder_api)
                .put(placeholder_api),
        )
        .nest_service(
            "/",
            ServeDir::new(STATIC_DIR).not_found_service(ServeFile::new(INDEX_FILE)),
        );

    // TODO improve address handling
    let ip = Ipv4Addr::new(0, 0, 0, 0);
    let port = 42002;

    let addr = SocketAddr::from((ip, port));

    log::info!("Listening on http://127.0.0.1:{port}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn placeholder_api() -> impl IntoResponse {
    "Egiraffe API goes here (todo)"
}
