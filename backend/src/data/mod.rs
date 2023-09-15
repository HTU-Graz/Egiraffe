use std::sync::Arc;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub first_names: Arc<str>,
    pub last_name: Arc<str>,
    pub password_hash: Arc<str>,
    pub totp_secret: Option<Arc<str>>,
    pub emails: Arc<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
}
