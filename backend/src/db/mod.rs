pub mod course;
pub mod init;
pub mod session;
pub mod university;
pub mod upload;
pub mod user;

use anyhow::Context;
use sqlx::{postgres::PgPoolOptions, Acquire, Executor, PgConnection, Pool, Postgres};
use std::env;
use tokio::fs::read_to_string;

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

pub async fn reset_and_init(db_pool: &Pool<Postgres>) -> anyhow::Result<()> {
    log::info!("Resetting and initializing database");

    let pool_connection = &mut db_pool.acquire().await?;
    let db_con = pool_connection.acquire().await?;
    yeet_everything(db_con).await?;

    let mut tx = db_pool.begin().await?;
    let db_con = tx.acquire().await?;

    create_schema(db_con).await?;

    init::create_universities(db_con).await?;
    init::create_email_states(db_con).await?;
    init::create_admin_users(db_pool).await?;

    tx.commit().await?;

    log::info!("Database reset and initialized");

    Ok(())
}

async fn create_schema(db_con: &mut PgConnection) -> Result<(), anyhow::Error> {
    log::info!("Creating schema");

    let query = read_to_string("../design/database/egiraffe-schema-generated.sql").await?;
    db_con.execute(&*query).await?;

    Ok(())
}

async fn yeet_everything(db_con: &mut PgConnection) -> Result<(), anyhow::Error> {
    log::warn!("Dropping everything (yeet)");

    const RESET_SEQUENCE: [&str; 4] = [
        "DROP SCHEMA public CASCADE;",
        "CREATE SCHEMA public;",
        "GRANT ALL ON SCHEMA public TO postgres;",
        "GRANT ALL ON SCHEMA public TO public;",
    ];

    for query in RESET_SEQUENCE {
        sqlx::query(query).execute(&mut *db_con).await?;
    }

    Ok(())
}
