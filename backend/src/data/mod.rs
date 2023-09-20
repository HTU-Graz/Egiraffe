use std::sync::Arc;

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
pub struct UserWithEmails {
    pub id: Uuid,
    pub first_names: Arc<str>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Course {
    pub id: Uuid,
    pub name: String,

    /// The ID of the university this course belongs to
    pub held_at: Uuid,
}
