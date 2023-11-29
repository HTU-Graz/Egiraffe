use std::{ops::Deref, sync::Arc};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct User {
//     pub id: Uuid,
//     pub first_names: Arc<str>,
//     pub last_name: Arc<str>,
//     pub password_hash: Arc<str>,
//     pub totp_secret: Option<Arc<str>>,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub first_names: Option<String>,
    pub last_name: Option<String>,
    pub password_hash: String,
    pub totp_secret: Option<String>,
    // pub emails: Vec<String>,
    /// The user's role in the system.
    ///
    /// Value | Meaning       | Notes
    /// :---- | :------------ | :--------------------------------
    /// 0     | Not logged in | Not for use in `user_role` column
    /// 1     | User          | Default, can self-register
    /// 2     | Moderator     | Can delete posts
    /// 3     | Admin         | Can delete users & edit user roles
    ///
    /// An `enum` would be better, but it's not supported by SQLx,
    /// at least not in a meaningful/simple way.
    pub user_role: i16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedactedUser {
    pub id: Uuid,
    pub first_names: Option<String>,
    pub last_name: Option<String>,
    pub totp_enabled: bool,
    // pub emails: Vec<String>,
    /// The user's role in the system.
    ///
    /// Value | Meaning       | Notes
    /// :---- | :------------ | :--------------------------------
    /// 0     | Not logged in | Not for use in `user_role` column
    /// 1     | User          | Default, can self-register
    /// 2     | Moderator     | Can delete posts
    /// 3     | Admin         | Can delete users & edit user roles
    ///
    /// An `enum` would be better, but it's not supported by SQLx,
    /// at least not in a meaningful/simple way.
    pub user_role: i16,
}

impl From<User> for RedactedUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            first_names: user.first_names,
            last_name: user.last_name,
            totp_enabled: user.totp_secret.is_some(),
            user_role: user.user_role,
        }
    }
}

impl From<UserWithEmails> for RedactedUser {
    fn from(value: UserWithEmails) -> Self {
        Self {
            id: value.id,
            first_names: Some(value.first_names.deref().into()),
            last_name: Some(value.last_name.deref().into()),
            totp_enabled: value.totp_secret.is_some(),
            user_role: value.user_role,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserWithEmails {
    pub id: Uuid,
    pub first_names: Arc<str>, // TODO those darn `Arc`s again; do we really need them?
    pub last_name: Arc<str>,
    pub password_hash: Arc<str>,
    pub totp_secret: Option<Arc<str>>,
    pub emails: Arc<Vec<String>>,
    /// The user's role in the system.  
    ///
    /// Value | Meaning       | Notes
    /// :---- | :------------ | :--------------------------------
    /// 0     | Not logged in | Not for use in `user_role` column
    /// 1     | User          | Default, can self-register
    /// 2     | Moderator     | Can delete posts
    /// 3     | Admin         | Can delete users & edit user roles
    ///
    /// An `enum` would be better, but it's not supported by SQLx,
    /// at least not in a meaningful/simple way.
    pub user_role: i16,
}

/// A token is a 256-bit (32-byte) value of random data.
///
/// It's used in a URL-safe base64 encoding in the database
/// and in a cookie called `session_token` in the browser.
pub type Token = [u8; 32];

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    /// Named `of_user` in the database
    pub user_id: Uuid,
    pub token: Token,
}

#[derive(Debug, Serialize)]
pub struct University<'a> {
    pub id: Uuid,
    pub full_name: &'static str,
    pub mid_name: &'static str,
    pub short_name: &'static str,
    pub domain_names: &'a [String],
}

// HACK this should not exist twice
#[derive(Debug, Serialize)]
pub struct OwnedUniversity {
    pub id: Uuid,
    pub full_name: String,
    pub mid_name: String,
    pub short_name: String,
    pub domain_names: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Course {
    pub id: Uuid,
    pub name: String,

    /// The ID of the university this course belongs to
    pub held_at: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Upload {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: i16,
    pub uploader: Uuid,
    pub upload_date: NaiveDateTime,
    pub last_modified_date: NaiveDateTime,

    /// The ID of the course this upload belongs to
    pub belongs_to: Uuid,

    /// The ID of the prof that held the course this upload belongs to
    pub held_by: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Prof {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Purchase {
    pub id: Uuid,
    pub buyer: Uuid,
    pub upload: Uuid,
    pub ecs_spent: i16,
    pub purchase_date: NaiveDateTime,
    pub rating: Option<i16>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemTransaction {
    pub affected_user: Uuid,
    pub transaction_date: NaiveDateTime,

    /// The amount of ECS the user gained or lost
    pub delta_ec: i16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub id: Uuid,
    pub name: String,
    pub mime_type: String,
    pub size: i64,
    // The latest one should match the file's last modified date
    pub revision_at: NaiveDateTime,
    /// The ID of the upload this file belongs to
    pub upload_id: Uuid,
}
