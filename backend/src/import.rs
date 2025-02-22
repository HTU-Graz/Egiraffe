use anyhow::Context;
use sqlx::{MySql, Pool, Postgres};

use crate::{
    data::University,
    db::{self, DB_POOL},
};

pub async fn perform_import() -> anyhow::Result<()> {
    #[cfg(feature = "prod")]
    core::panic!("This is the import feature, which is not available in production");

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

    let import_db_pool = db::connect_import()
        .await
        .context("Import DB connection failed")?;
    log::info!("Connected to import database");

    log::info!("Starting import");

    import_universities(&db_pool, &import_db_pool).await?;

    log::info!("Import done");

    Ok(())
}

async fn import_universities(
    target_db: &Pool<Postgres>,
    source_db: &Pool<MySql>,
) -> anyhow::Result<()> {
    log::info!("Importing universities");

    let mut tx = target_db.begin().await?;

    #[derive(Debug, sqlx::FromRow)]
    struct LegacyUniversity {
        /// Mapped to [`University::short_name`]
        name_kurz: String,
        /// Mapped to [`University::full_name`]
        name_lang: String,
        /// Mapped to [`University::mid_name``]
        name_mittel: String,
        /// Website of the university
        homepage: String,
        /// Campus management system homepage
        cms_homepage: String,
        /// Background color of the university
        farbcode: String,
        /// Text color of the university
        farbcode_text: String,
    }

    let unis = sqlx::query(
        r#"
        SELECT
            *
        FROM
            egiraffe_studium_universities
        "#,
    )
    .fetch_all(source_db)
    .await
    .context("Failed to fetch universities")?;

    let mut unis_new: Vec<University> = Vec::new();

    dbg!(&unis);

    Ok(())
}
