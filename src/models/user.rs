use serde::{Deserialize, Serialize};
use crate::utils::Snowflake;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Snowflake,
    pub username: String,
    #[serde(default)]
    pub discriminator: String,
    #[serde(default)]
    pub global_name: Option<String>,
    pub avatar: Option<String>,
    #[serde(default)]
    pub bot: bool,
    #[serde(default)]
    pub system: bool,
    #[serde(default)]
    pub mfa_enabled: Option<bool>,
    pub banner: Option<String>,
    pub accent_color: Option<u32>,
    pub locale: Option<String>,
    pub verified: Option<bool>,
    pub email: Option<String>,
    pub flags: Option<u64>,
    pub premium_type: Option<u8>,
    pub public_flags: Option<u64>,
    pub bio: Option<String>,
}

impl User {
    pub fn tag(&self) -> String {
        if self.discriminator == "0" || self.discriminator.is_empty() {
            self.username.clone()
        } else {
            format!("{}#{}", self.username, self.discriminator)
        }
    }

    pub fn mention(&self) -> String {
        format!("<@{}>", self.id)
    }

    pub fn avatar_url(&self) -> Option<String> {
        self.avatar.as_ref().map(|hash| {
            let extension = if hash.starts_with("a_") { "gif" } else { "png" };
            format!(
                "https://cdn.discordapp.com/avatars/{}/{}.{}",
                self.id, hash, extension
            )
        })
    }

    pub fn default_avatar_url(&self) -> String {
        let index = if self.discriminator == "0" {
            (self.id.0 >> 22) % 6
        } else {
            self.discriminator.parse::<u64>().unwrap_or(0) % 5
        };
        format!(
            "https://cdn.discordapp.com/embed/avatars/{}.png",
            index
        )
    }

    pub fn display_avatar_url(&self) -> String {
        self.avatar_url().unwrap_or_else(|| self.default_avatar_url())
    }
}

