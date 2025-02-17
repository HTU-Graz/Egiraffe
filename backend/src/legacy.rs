//! Backwards compatibility with the old Egiraffe (written in PHP) database

pub struct LegacyId {
    pub id: i32,
    pub table: u8,
}
