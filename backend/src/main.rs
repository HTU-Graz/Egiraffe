#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::items_after_statements,
    clippy::module_name_repetitions,
    clippy::unused_async
)]

mod api;
mod data;
mod db;
mod util;

use std::{
    env,
    fs::canonicalize,
    net::{Ipv4Addr, SocketAddr},
};

use anyhow::Context;
use axum::Router;
use sqlx::{Pool, Postgres};
use tower_http::services::{ServeDir, ServeFile};

use crate::db::DB_POOL;

// Make sure to build the frontend first!
const STATIC_DIR: &str = "../frontend/dist";
const INDEX_FILE: &str = "../frontend/dist/index.html";

// TODO improve address handling
const IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 42);
const PORT: u16 = 42002;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    // Prepare the database
    let db_pool = db::connect().await.context("DB connection failed")?;
    DB_POOL.set(Box::leak(Box::new(db_pool))).unwrap();
    let db_pool = *DB_POOL.get().unwrap();
    log::info!("Connected to database");

    sqlx::migrate!().run(db_pool).await.unwrap();
    log::info!("Database migrations completed");

    #[cfg(debug_assertions)]
    if env::var("NO_DEFAULT_ENTRIES").is_err() {
        db::DEBUG_insert_default_entries(&db_pool).await?;
    }

    let static_files = ServeDir::new(STATIC_DIR).not_found_service(ServeFile::new(INDEX_FILE));
    log::info!(
        "Serving static files from {STATIC_DIR}, canonicalized to {}",
        canonicalize(STATIC_DIR)?.display()
    );

    let app = Router::new()
        .nest("/api", api::routes())
        .nest_service("/", static_files);

    let addr = SocketAddr::from((IP, PORT));

    log::info!("Listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .context("Failed to start server")
        .unwrap();

    Ok(())
}
