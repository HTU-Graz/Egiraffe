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
mod conf;

use std::{
    env,
    fs::canonicalize,
    net::SocketAddr,
};

use anyhow::Context;
use axum::Router;
use sqlx::{Pool, Postgres};
use tower_http::services::{ServeDir, ServeFile};
use owo_colors::OwoColorize;

use crate::db::DB_POOL;
use crate::conf::CONF;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    #[cfg(not(feature = "prod"))]
    {
        println!("{} Never use this in Production!", "DEBUG Mode!".on_red());
        log::warn!("DEBUG Mode! Never use this in Production!");
    }

    // Prepare the database
    let db_pool = db::connect().await.context("DB connection failed")?;
    DB_POOL.set(Box::leak(Box::new(db_pool))).unwrap();
    let db_pool = *DB_POOL.get().unwrap();
    log::info!("Connected to database");

    sqlx::migrate!().run(db_pool).await.unwrap();
    log::info!("Database migrations completed");

    #[cfg(not(feature = "prod"))]
    if CONF.database.debugdefaultentries {
        db::debug_insert_default_entries(&db_pool).await?;
    }

    let static_files = ServeDir::new(&CONF.webserver.staticdir).not_found_service(ServeFile::new(&CONF.webserver.indexfile));
    log::info!(
        "Serving static files from {}, canonicalized to {}",
        &CONF.webserver.staticdir,
        canonicalize(&CONF.webserver.staticdir)?.display()
    );

    let app = Router::new()
        .nest("/api", api::routes())
        .nest_service("/", static_files);

    let addr = SocketAddr::from((CONF.webserver.ip, CONF.webserver.port));

    log::info!("Listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .context("Failed to start server")
        .unwrap();

    Ok(())
}
