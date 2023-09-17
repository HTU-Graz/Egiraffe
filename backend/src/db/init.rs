use sqlx::{self, Acquire, PgConnection, Pool, Postgres};
use uuid::Uuid;

use crate::data::University;

pub async fn create_universities(db_con: &mut PgConnection) -> anyhow::Result<()> {
    log::info!("Creating universities");

    let unis = [
        University {
            id: Uuid::new_v4(),
            full_name: "Technische Universität Graz",
            mid_name: "TU Graz",
            short_name: "TUG",
            domain_names: &["tugraz.at".to_string(), "student.tugraz.at".to_string()],
        },
        University {
            id: Uuid::new_v4(),
            full_name: "Karl Franzens Universität Graz",
            mid_name: "Uni Graz",
            short_name: "KFU",
            domain_names: &["uni-graz.at".to_string()],
        },
    ];

    for uni in unis {
        let University {
            id,
            full_name,
            mid_name,
            short_name,
            domain_names,
        } = uni;

        sqlx::query!(
            r#"
                    INSERT INTO university (id, name_full, name_mid, name_short, domain_names)
                    VALUES ($1, $2, $3, $4, $5)
                "#,
            id,
            full_name,
            mid_name,
            short_name,
            &domain_names
        )
        .execute(&mut *db_con)
        .await?;
    }

    Ok(())
}

pub async fn create_email_states(db_con: &mut PgConnection) -> anyhow::Result<()> {
    log::info!("Creating email states");

    let email_states = ["unverified", "verified"];

    for state in email_states {
        sqlx::query!(
            r#"
                    INSERT INTO email_status (status)
                    VALUES ($1)
                "#,
            state
        )
        .execute(&mut *db_con)
        .await?;
    }

    Ok(())
}
