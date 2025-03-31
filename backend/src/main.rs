#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::items_after_statements,
    clippy::module_name_repetitions,
    clippy::unused_async
)]

mod api;
mod conf;
mod data;
mod db;
mod legacy;
mod mail;
mod util;

#[cfg(feature = "import")]
mod import;

use std::{fs::canonicalize, net::SocketAddr};

use anyhow::Context;
use axum::Router;
use owo_colors::OwoColorize;
use tower_http::services::{ServeDir, ServeFile};

use crate::conf::CONF;
use crate::db::DB_POOL;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(not(feature = "import"))]
    {
        server().await
    }

    #[cfg(feature = "import")]
    import::perform_import().await
}

async fn server() -> anyhow::Result<()> {
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

    // Prepare Mail system
    mail::init();

    sqlx::migrate!().run(db_pool).await.unwrap();
    log::info!("Database migrations completed");

    #[cfg(not(feature = "prod"))]
    if CONF.database.debugdefaultentries {
        let mut tx = db_pool.begin().await?;
        db::debug_insert_default_entries(&mut tx).await?;
        tx.commit().await?;
    }

    log::info!(
        "Serving static files from {}, canonicalized to {}",
        &CONF.webserver.staticdir,
        canonicalize(&CONF.webserver.staticdir)?.display()
    );

    let static_files = ServeDir::new(&CONF.webserver.staticdir)
        .not_found_service(ServeFile::new(&CONF.webserver.indexfile));

    let app = Router::new()
        .nest("/api", api::routes())
        .fallback_service(static_files);

    let addr = SocketAddr::from((CONF.webserver.ip, CONF.webserver.port));

    log::info!("Listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .context("Failed to start server")
        .unwrap();

    Ok(())
}
