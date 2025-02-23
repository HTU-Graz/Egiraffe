use anyhow::Context;
use sqlx::{MySql, PgTransaction, Pool, Postgres};

use crate::{
    data::{OwnedUniversity, RgbColor, University},
    db::{self, DB_POOL},
    legacy::{self, LegacyTable},
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

    let mut tx = db_pool.begin().await?;

    import_universities(&mut tx, &import_db_pool).await?;

    tx.commit().await?;
    log::info!("Import done");

    Ok(())
}

async fn import_universities(
    mut target_db: &mut PgTransaction<'_>,
    source_db: &Pool<MySql>,
) -> anyhow::Result<()> {
    log::info!("Importing universities");

    #[derive(Debug, sqlx::FromRow)]
    struct LegacyUniversity {
        id: i32,
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

    let unis: Vec<LegacyUniversity> = sqlx::query_as(
        r#"
        SELECT
            id,
            name_kurz,
            name_lang,
            name_mittel,
            homepage,
            cms_homepage,
            farbcode,
            farbcode_text
        FROM
            egiraffe_studium_universities
        "#,
    )
    .fetch_all(source_db)
    .await
    .context("Failed to fetch universities")?;

    let mut unis_new: Vec<OwnedUniversity> = Vec::with_capacity(unis.len());

    fn hex_to_rgb(hex: &str) -> anyhow::Result<RgbColor> {
        let r = u8::from_str_radix(&hex[0..2], 16)?;
        let g = u8::from_str_radix(&hex[2..4], 16)?;
        let b = u8::from_str_radix(&hex[4..6], 16)?;

        Ok(RgbColor { r, g, b })
    }

    for uni in unis {
        let background_color = hex_to_rgb(&uni.farbcode)?;
        let text_color = hex_to_rgb(&uni.farbcode_text)?;

        let id = legacy::LegacyId {
            id: uni.id,
            table: LegacyTable::University,
        };

        unis_new.push(OwnedUniversity {
            id: id.try_into()?,
            full_name: uni.name_lang,
            mid_name: uni.name_mittel,
            short_name: uni.name_kurz,
            email_domain_names: Vec::new(),
            homepage_url: uni.homepage,
            cms_url: uni.cms_homepage,
            background_color,
            text_color,
        });
    }

    for mut uni in unis_new {
        if uni.mid_name == "Uni Innsbruck" {
            uni.short_name = "UI".to_string();
        }

        db::university::create_university_with_id(&mut target_db, uni).await?;
    }

    Ok(())
}
