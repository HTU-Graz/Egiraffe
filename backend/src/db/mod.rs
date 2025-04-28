pub mod course;
pub mod ecs;
pub mod file;
pub mod init;
pub mod prof;
pub mod purchase;
pub mod session;
pub mod university;
pub mod upload;
pub mod user;

use crate::conf::CONF;
use anyhow::Context;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use sqlx::{
    postgres::PgPoolOptions, Acquire, Executor, PgConnection, PgTransaction, Pool, Postgres,
};
use tokio::fs::read_to_string;

pub static DB_POOL: OnceCell<&'static sqlx::PgPool> = OnceCell::new();

pub async fn connect() -> anyhow::Result<Pool<Postgres>> {
    log::info!("Connecting to database at {}", CONF.database.url);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&CONF.database.url)
        .await?;

    Ok(pool)
}

#[cfg(feature = "import")]
pub async fn connect_import() -> anyhow::Result<Pool<sqlx::MySql>> {
    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&CONF.import.url)
        .await?;

    Ok(pool)
}

#[derive(sqlx::FromRow)]
struct SelectExistsTmp {
    exists: Option<bool>,
}

struct SelectExists(bool);

impl From<SelectExistsTmp> for SelectExists {
    fn from(tmp: SelectExistsTmp) -> Self {
        Self(tmp.exists.unwrap_or(false))
    }
}

pub async fn debug_insert_default_entries(mut tx: &mut PgTransaction<'_>) -> anyhow::Result<()> {
    // TODO make sure this works when half ot it is already initialized
    init::debug_create_universities(&mut tx).await;

    init::debug_create_admin_users(&mut tx).await;

    log::info!("Database reset and initialized");

    Ok(())
}

#[derive(Debug, Deserialize)]
pub enum SortOrder {
    Ascending,
    Descending,
}
