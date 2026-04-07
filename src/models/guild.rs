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


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildMember {
    pub user: Option<crate::models::user::User>,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub roles: Vec<Snowflake>,
    pub joined_at: String,
    pub premium_since: Option<String>,
    pub deaf: bool,
    pub mute: bool,
    pub pending: Option<bool>,
    pub communication_disabled_until: Option<String>,
}

impl GuildMember {
    /// Returns the member's display name (nickname if set, otherwise username).
    pub fn display_name(&self) -> &str {
        if let Some(ref nick) = self.nick {
            nick.as_str()
        } else if let Some(ref user) = self.user {
            user.username.as_str()
        } else {
            "Unknown"
        }
    }

    /// Returns the member's mention string.
    pub fn mention(&self) -> String {
        if let Some(ref user) = self.user {
            format!("<@{}>", user.id)
        } else {
            String::new()
        }
    }

    /// Returns true if the member has the given role ID.
    pub fn has_role(&self, role_id: Snowflake) -> bool {
        self.roles.contains(&role_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: Snowflake,
    pub name: String,
    pub color: u32,
    pub hoist: bool,
    pub icon: Option<String>,
    pub unicode_emoji: Option<String>,
    pub position: u32,
    pub permissions: String,
    pub managed: bool,
    pub mentionable: bool,
}

impl Role {
    /// Returns the role's mention string.
    pub fn mention(&self) -> String {
        format!("<@&{}>", self.id)
    }

    /// Returns the role's color as a hex string (e.g. `#FF0000`).
    pub fn color_hex(&self) -> String {
        format!("#{:06X}", self.color)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ban {
    pub reason: Option<String>,
    pub user: crate::models::user::User,
}
