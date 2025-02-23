use anyhow::Context;
use sqlx::{PgPool, PgTransaction};
use uuid::Uuid;

use crate::data::{DbRgbColor, OwnedUniversity, RgbColor};

pub async fn get_universities(db_pool: &PgPool) -> anyhow::Result<Vec<OwnedUniversity>> {
    sqlx::query!(
        r#"
        SELECT
            id,
            name_full,
            name_mid,
            name_short,
            email_domain_names,
            homepage_url,
            cms_url,
            background_color AS "background_color: DbRgbColor",
            text_color AS "text_color: DbRgbColor"
        FROM
            universities
        "#,
    )
    .fetch_all(db_pool)
    .await
    .context("Failed to get courses")
    .map(|unis| {
        unis.into_iter()
            .map(|uni| OwnedUniversity {
                id: uni.id,
                full_name: uni.name_full,
                mid_name: uni.name_mid,
                short_name: uni.name_short,
                email_domain_names: uni.email_domain_names,
                homepage_url: uni.homepage_url,
                cms_url: uni.cms_url,
                background_color: uni.background_color.into(),
                text_color: uni.text_color.into(),
            })
            .collect()
    })
}

/// Create a university, returning its ID, ignoring the ID in the input
pub async fn create_university(
    tx: &mut PgTransaction<'_>,
    university: OwnedUniversity,
) -> anyhow::Result<Uuid> {
    #[derive(Debug, sqlx::Type)]
    struct Oida {
        id: Uuid,
    };

    let id = sqlx::query_as!(
        Oida,
        r#"
        INSERT INTO universities (
            name_full,
            name_mid,
            name_short,
            email_domain_names,
            homepage_url,
            cms_url,
            background_color,
            text_color
        )
        VALUES (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6,
            $7,
            $8
        )
        RETURNING
            id
        "#,
        university.full_name,
        university.mid_name,
        university.short_name,
        &university.email_domain_names,
        university.homepage_url,
        university.cms_url,
        DbRgbColor::from(university.background_color) as _,
        DbRgbColor::from(university.text_color) as _,
    )
    .fetch_one(&mut **tx)
    .await
    .context("Failed to create university")?;

    Ok(id.id)
}

/// Create a university with a specific ID
pub async fn create_university_with_id(
    tx: &mut PgTransaction<'_>,
    university: OwnedUniversity,
) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        INSERT INTO universities (
            id,
            name_full,
            name_mid,
            name_short,
            email_domain_names,
            homepage_url,
            cms_url,
            background_color,
            text_color
        )
        VALUES (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6,
            $7,
            $8,
            $9
        )
        "#,
    )
    .bind(university.id)
    .bind(university.full_name)
    .bind(university.mid_name)
    .bind(university.short_name)
    .bind(&university.email_domain_names)
    .bind(university.homepage_url)
    .bind(university.cms_url)
    .bind(DbRgbColor::from(university.background_color))
    .bind(DbRgbColor::from(university.text_color))
    .execute(&mut **tx)
    .await
    .context("Failed to create university")?;

    Ok(())
}
