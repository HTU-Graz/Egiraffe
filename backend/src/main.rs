mod api;
mod data;
mod db;

use std::{
    fs::canonicalize,
    net::{Ipv4Addr, SocketAddr},
};

use axum::Router as AxumRouter;
use sqlx::{Pool, Postgres};
use tower_http::services::{ServeDir, ServeFile};

// Make sure to build the frontend first!
const STATIC_DIR: &str = "../frontend/dist";
const INDEX_FILE: &str = "../frontend/dist/index.html";

// TODO improve address handling
const IP: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);
const PORT: u16 = 42002;

type AppState = Pool<Postgres>;
type Router = AxumRouter<AppState>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    log::info!("Connecting to database");
    let db_pool = db::connect().await?;
    log::info!("Connected to database");

    db::reset_and_init(&db_pool).await?;

    let static_files = ServeDir::new(STATIC_DIR).not_found_service(ServeFile::new(INDEX_FILE));
    log::info!(
        "Serving static files from {}, canonicalized to {}",
        STATIC_DIR,
        canonicalize(STATIC_DIR)?.display()
    );

    let app = Router::new()
        .nest("/api", api::routes())
        .nest_service("/", static_files)
        .with_state(db_pool);

    let addr = SocketAddr::from((IP, PORT));

    // db::demo(&db_pool).await?;

    log::info!("Listening on http://127.0.0.1:{PORT}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
