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

use anyhow::Context;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, Acquire, Executor, PgConnection, Pool, Postgres};
use std::env;
use tokio::fs::read_to_string;

pub static DB_POOL: OnceCell<&'static sqlx::PgPool> = OnceCell::new();

pub async fn connect() -> anyhow::Result<Pool<Postgres>> {
    let pool =
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&env::var("DATABASE_URL").context(
                "Needs the env var DATABASE_URL set to the connection string of the pg db",
            )?)
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

pub async fn insert_default_entries(db_pool: &Pool<Postgres>) -> anyhow::Result<()> {
    let mut tx = db_pool.begin().await?;
    let db_con = tx.acquire().await?;

    // TODO make sure this works when half ot it is already initialized
    let _res = init::create_universities(db_con).await;
    let _res = init::create_email_states(db_con).await;
    tx.commit().await?;

    let mut tx = db_pool.begin().await?;
    let db_con = tx.acquire().await?;
    let _res = init::create_admin_users(db_pool).await;
    tx.commit().await?;

    log::info!("Database reset and initialized");

    Ok(())
}

#[derive(Debug, Deserialize)]
pub enum SortOrder {
    Ascending,
    Descending,
}
