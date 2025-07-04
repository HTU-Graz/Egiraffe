use std::{ops::Deref, sync::Arc};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
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
            totp_enabled: value.totp_secret.is_some(), //TODO: Do we want that to be publically exposed? Or just for a user himself?? Shall we create Implementations here?
            user_role: value.user_role,
        }
    }
}

//TODO: Also implement E-Mail-Address statuses as struct and use it here.
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
    pub nick: Option<String>,
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
    pub email_domain_names: &'a [String],
}

// HACK this should not exist twice
#[derive(Debug, Serialize)]
pub struct OwnedUniversity {
    pub id: Uuid,
    pub full_name: String,
    pub mid_name: String,
    pub short_name: String,
    pub email_domain_names: Vec<String>,
    pub homepage_url: String,
    pub cms_url: String,
    pub background_color: RgbColor,
    pub text_color: RgbColor,
}

/// An RGB color struct as three [`u8`]s, but we have to pretend they're [`i8`]s
/// because SQLx (and especially Postgres) doesn't support unsigned types
///
/// Use [`RgbColor`] instead of this struct, and its [`From`] and [`Into`] implementations
#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "rgb_color")]
pub struct DbRgbColor {
    r: i8,
    g: i8,
    b: i8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<DbRgbColor> for RgbColor {
    fn from(db_color: DbRgbColor) -> Self {
        Self {
            r: db_color.r as u8,
            g: db_color.g as u8,
            b: db_color.b as u8,
        }
    }
}

impl From<RgbColor> for DbRgbColor {
    fn from(color: RgbColor) -> Self {
        Self {
            r: color.r as i8,
            g: color.g as i8,
            b: color.b as i8,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Course {
    pub id: Uuid,
    pub name: String,

    /// The ID of the university this course belongs to
    pub held_at: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Upload {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: i16,
    pub uploader: Uuid, // TODO consider adding resolved values for faster API times
    pub upload_date: NaiveDateTime,
    pub last_modified_date: NaiveDateTime,

    /// The date associated with the upload, e.g. the date of the exam (nullable)
    pub associated_date: Option<NaiveDateTime>,

    pub upload_type: UploadType,

    /// The ID of the course this upload belongs to
    pub belongs_to: Uuid, // TODO consider adding resolved values for faster API times

    /// The ID of the prof that held the course this upload belongs to
    pub held_by: Option<Uuid>, // TODO consider adding resolved values for faster API times
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Prof {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Purchase {
    pub user_id: Uuid,
    pub upload_id: Uuid,
    pub ecs_spent: i16,
    pub purchase_date: NaiveDateTime,
    pub rating: Option<i16>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct File {
    pub id: Uuid,
    pub name: String,
    pub mime_type: String,
    pub size: i64,
    /// The SHA3-256 digest of the file's contents (it's delicious)
    pub sha3_256: String,
    // The latest one should match the file's last modified date
    pub revision_at: NaiveDateTime,
    /// The ID of the upload this file belongs to
    pub upload_id: Uuid,
    pub approval_uploader: bool,
    pub approval_mod: bool,
}

#[derive(sqlx::Type, Debug, Serialize, Deserialize, Clone)]
#[sqlx(type_name = "upload_type_enum", rename_all = "snake_case")]
pub enum UploadType {
    Exam,
    ExamPrep,
    CourseSummary,
    Homework,
    LectureNotes,
    QuestionCollection,
    Protocol,
    Other,
    Script,
    Presentation,
    Unknown,
}

impl UploadType {
    pub fn to_de_string(&self) -> &'static str {
        match self {
            UploadType::Exam => "Klausurangabe",
            UploadType::ExamPrep => "Prüfungsfragenausarbeitung",
            UploadType::CourseSummary => "Stoffzusammenfassung",
            UploadType::Homework => "Hausübung",
            UploadType::LectureNotes => "Mitschrift",
            UploadType::QuestionCollection => "Fragensammlung",
            UploadType::Protocol => "Protokoll",
            UploadType::Other => "Sonstiges",
            UploadType::Script => "Skriptum",
            UploadType::Presentation => "Präsentation",
            UploadType::Unknown => "kein Typ",
        }
    }
}

// Full texts
// 	ID 	Bezeichnung 	Farbe
// 	Edit Edit 	Copy Copy 	Delete Delete 	0 	kein Typ 	#FFFFFF
// 	Edit Edit 	Copy Copy 	Delete Delete 	1 	Sonstiges 	#777777
// 	Edit Edit 	Copy Copy 	Delete Delete 	2 	Klausurangabe 	#FF0000
// 	Edit Edit 	Copy Copy 	Delete Delete 	3 	Fragensammlung 	#ab2486
// 	Edit Edit 	Copy Copy 	Delete Delete 	4 	Prüfungsfragenausarbeitung 	#00FFFF
// 	Edit Edit 	Copy Copy 	Delete Delete 	5 	Mitschrift 	#0000FF
// 	Edit Edit 	Copy Copy 	Delete Delete 	6 	Stoffzusammenfassung 	#FF00FF
// 	Edit Edit 	Copy Copy 	Delete Delete 	7 	Protokoll 	#000000
// 	Edit Edit 	Copy Copy 	Delete Delete 	8 	Skriptum 	#AA9955
// 	Edit Edit 	Copy Copy 	Delete Delete 	9 	Hausübung 	#3388FF
// 	Edit Edit 	Copy Copy 	Delete Delete 	10 	Präsentation 	#33FF33

// impl
