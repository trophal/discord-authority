use serde::{Deserialize, Serialize};
use crate::utils::Snowflake;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guild {
    pub id: Snowflake,
    pub name: String,
    pub icon: Option<String>,
    pub splash: Option<String>,
    pub discovery_splash: Option<String>,
    pub owner_id: Snowflake,
    pub region: Option<String>,
    pub afk_channel_id: Option<Snowflake>,
    pub afk_timeout: u32,
    pub verification_level: u8,
    pub default_message_notifications: u8,
    pub explicit_content_filter: u8,
    pub features: Vec<String>,
    pub mfa_level: u8,
    pub system_channel_id: Option<Snowflake>,
    pub system_channel_flags: u32,
    pub rules_channel_id: Option<Snowflake>,
    pub vanity_url_code: Option<String>,
    pub description: Option<String>,
    pub banner: Option<String>,
    pub premium_tier: u8,
    pub premium_subscription_count: Option<u32>,
    pub preferred_locale: String,
    pub public_updates_channel_id: Option<Snowflake>,
    pub nsfw_level: u8,
    pub premium_progress_bar_enabled: bool,
}

impl Guild {
    pub fn icon_url(&self) -> Option<String> {
        self.icon.as_ref().map(|hash| {
            let extension = if hash.starts_with("a_") { "gif" } else { "png" };
            format!(
                "https://cdn.discordapp.com/icons/{}/{}.{}",
                self.id, hash, extension
            )
        })
    }

    pub fn banner_url(&self) -> Option<String> {
        self.banner.as_ref().map(|hash| {
            let extension = if hash.starts_with("a_") { "gif" } else { "png" };
            format!(
                "https://cdn.discordapp.com/banners/{}/{}.{}",
                self.id, hash, extension
            )
        })
    }
}

