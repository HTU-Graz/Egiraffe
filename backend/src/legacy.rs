//! Backwards compatibility with the old Egiraffe (written in PHP) database

use anyhow::ensure;
use uuid::Uuid;

pub struct LegacyId {
    pub id: i32,
    pub table: LegacyTable,
}

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum LegacyTable {
    University = 0,
    Course = 1,
    Prof = 2,
    Upload = 3,
    File = 4,
    User = 5,
    Email = 6,
}

impl TryFrom<u8> for LegacyTable {
    type Error = anyhow::Error; // HACK consider using thiserror instead

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::University),
            1 => Ok(Self::Course),
            2 => Ok(Self::Prof),
            3 => Ok(Self::Upload),
            4 => Ok(Self::File),
            5 => Ok(Self::User),
            6 => Ok(Self::Email),
            _ => Err(anyhow::anyhow!("Invalid legacy table id: {value}")),
        }
    }
}

impl TryFrom<Uuid> for LegacyId {
    type Error = anyhow::Error; // HACK consider using thiserror instead

    fn try_from(input_uuid: Uuid) -> Result<Self, Self::Error> {
        ensure!(
            input_uuid.get_version_num() == 8,
            "Legacy UUIDs must be version 8"
        );

        ensure!(
            input_uuid.get_variant() == uuid::Variant::RFC4122,
            "Legacy UUIDs must be RFC4122"
        );

        let bytes = input_uuid.as_bytes();

        // Octet 5 is the legacy table id
        let table = LegacyTable::try_from(bytes[5])?;

        // The last 4 octets are the legacy id, as a big-endian u32
        let id = i32::from_be_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]);

        Ok(Self { id, table })
    }
}

impl TryFrom<LegacyId> for Uuid {
    type Error = anyhow::Error; // HACK consider using thiserror instead

    fn try_from(legacy_id: LegacyId) -> Result<Self, Self::Error> {
        let mut bytes = [0u8; 16];

        // Version 8
        bytes[6] = 0b1000;

        // RFC4122
        bytes[8] = 0b1000;

        // Table id
        bytes[5] = legacy_id.table as u8;

        // Id
        bytes[12..16].copy_from_slice(&legacy_id.id.to_be_bytes());

        Ok(Uuid::from_bytes(bytes))
    }
}
