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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserWithEmails {
    pub id: Uuid,
    pub first_names: Arc<str>,
    pub last_name: Arc<str>,
    pub password_hash: Arc<str>,
    pub totp_secret: Option<Arc<str>>,
    pub emails: Arc<Vec<String>>,
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
