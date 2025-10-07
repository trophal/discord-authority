use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

use super::DISCORD_EPOCH;

/// Discord Snowflake ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Snowflake(pub u64);

impl Snowflake {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Get timestamp from snowflake
    pub fn timestamp(&self) -> u64 {
        (self.0 >> 22) + DISCORD_EPOCH
    }

    /// Get worker ID from snowflake
    pub fn worker_id(&self) -> u8 {
        ((self.0 & 0x3E0000) >> 17) as u8
    }

    /// Get process ID from snowflake
    pub fn process_id(&self) -> u8 {
        ((self.0 & 0x1F000) >> 12) as u8
    }

    /// Get increment from snowflake
    pub fn increment(&self) -> u16 {
        (self.0 & 0xFFF) as u16
    }
}

impl From<u64> for Snowflake {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

impl From<Snowflake> for u64 {
    fn from(sf: Snowflake) -> Self {
        sf.0
    }
}

impl From<String> for Snowflake {
    fn from(s: String) -> Self {
        Self(s.parse().unwrap_or(0))
    }
}

impl From<&str> for Snowflake {
    fn from(s: &str) -> Self {
        Self(s.parse().unwrap_or(0))
    }
}

impl fmt::Display for Snowflake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for Snowflake {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self(s.parse().unwrap_or(0)))
    }
}

