pub mod snowflake;

pub use snowflake::Snowflake;

use chrono::{DateTime, Utc};

/// Discord's epoch (2015-01-01T00:00:00.000Z)
pub const DISCORD_EPOCH: u64 = 1420070400000;

/// Get current timestamp in milliseconds
pub fn now() -> u64 {
    chrono::Utc::now().timestamp_millis() as u64
}

/// Convert timestamp to DateTime
pub fn timestamp_to_datetime(timestamp: u64) -> DateTime<Utc> {
    let seconds = (timestamp / 1000) as i64;
    let nanos = ((timestamp % 1000) * 1_000_000) as u32;
    DateTime::from_timestamp(seconds, nanos).unwrap_or_default()
}

