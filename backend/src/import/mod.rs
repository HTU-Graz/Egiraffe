use anyhow::Context;

use crate::{
    conf::CONF,
    db::{self, DB_POOL},
};

pub async fn perform_import() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    #[cfg(feature = "prod")]
    {
        core::panic!("This is the import feature, which is not available in production");
    }

    // Prepare the database
    let db_pool = db::connect().await.context("DB connection failed")?;
    DB_POOL.set(Box::leak(Box::new(db_pool))).unwrap();
    let db_pool = *DB_POOL.get().unwrap();
    log::info!("Connected to database");

    sqlx::migrate!().run(db_pool).await.unwrap();
    log::info!("Database migrations completed");

    db::debug_insert_default_entries(&db_pool).await?;

    let import_db_pool = db::connect_import()
        .await
        .context("Import DB connection failed")?;
    log::info!("Connected to import database");

    Ok(())
}
