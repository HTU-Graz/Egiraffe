use sqlx::{self, PgConnection, PgTransaction, Pool, Postgres};
use tokio::task::JoinSet;
use uuid::Uuid;

use crate::{
    api::v1::AuthLevel,
    data::{University, UserWithEmails},
};

use super::user::make_pwd_hash;

pub async fn debug_create_universities(db_con: &mut PgConnection) -> anyhow::Result<()> {
    log::info!("Creating universities");

    let unis = [
        University {
            id: "4e5f6c68-b966-4be1-9bf3-1ba9632deb74".try_into().unwrap(),
            full_name: "Technische Universität Graz",
            mid_name: "TU Graz",
            short_name: "TUG",
            email_domain_names: &["tugraz.at".to_string(), "student.tugraz.at".to_string()],
        },
        University {
            id: Uuid::new_v4(),
            full_name: "Karl Franzens Universität Graz",
            mid_name: "Uni Graz",
            short_name: "KFU",
            email_domain_names: &["uni-graz.at".to_string()],
        },
    ];

    for uni in unis {
        let University {
            id,
            full_name,
            mid_name,
            short_name,
            email_domain_names,
        } = uni;

        sqlx::query!(
            "
            INSERT INTO
                universities (
                    id,
                    name_full,
                    name_mid,
                    name_short,
                    email_domain_names
                )
            VALUES
                ($1, $2, $3, $4, $5)
            ",
            id,
            full_name,
            mid_name,
            short_name,
            &email_domain_names
        )
        .execute(&mut *db_con)
        .await?;
    }

    Ok(())
}

pub async fn debug_create_admin_users(mut tx: &mut PgTransaction<'_>) -> anyhow::Result<()> {
    log::info!("Creating admin users");

    let users = [
        UserWithEmails {
            id: Uuid::new_v4(),
            first_names: "Admin".to_string().into(),
            last_name: "Admin".to_string().into(),
            password_hash: make_pwd_hash("admin").into(),
            totp_secret: None,
            emails: vec!["admin@tugraz.at".to_string()].into(),
            user_role: AuthLevel::Admin,
            nick: Some("admin".to_string()),
        },
        UserWithEmails {
            id: Uuid::new_v4(),
            first_names: "Moderator".to_string().into(),
            last_name: "Moderator".to_string().into(),
            password_hash: make_pwd_hash("mod").into(),
            totp_secret: None,
            emails: vec!["mod@tugraz.at".to_string()].into(),
            user_role: AuthLevel::Moderator,
            nick: Some("mod".to_string()),
        },
        UserWithEmails {
            id: Uuid::new_v4(),
            first_names: "User".to_string().into(),
            last_name: "User".to_string().into(),
            password_hash: make_pwd_hash("user").into(),
            totp_secret: None,
            emails: vec!["user@tugraz.at".to_string()].into(),
            user_role: AuthLevel::RegularUser,
            nick: Some("test_user".to_string()),
        },
    ];

    // let mut join_set = JoinSet::new();

    for user in users {
        crate::db::user::register(&mut tx, user).await;
        // join_set.spawn(async move { crate::db::user::register(&mut tx, user).await });
    }

    // while let Some(res) = join_set.join_next().await {
    //     res??;
    // }

    Ok(())
}
