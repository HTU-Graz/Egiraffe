use anyhow::Context;
use sqlx::PgPool;

use crate::data::OwnedUniversity;

pub async fn get_universities(db_pool: &PgPool) -> anyhow::Result<Vec<OwnedUniversity>> {
    sqlx::query!(
        r#"
            SELECT id,
                name_full,
                name_mid,
                name_short,
                domain_names
            FROM university
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
                domain_names: uni.domain_names,
            })
            .collect()
    })
}

pub(crate) async fn create_university(
    db_pool: &&sqlx::Pool<sqlx::Postgres>,
    university: OwnedUniversity,
) -> anyhow::Result<()> {
    todo!()
}
