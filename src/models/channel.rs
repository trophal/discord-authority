use serde::{Deserialize, Serialize};
use crate::utils::Snowflake;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Channel {
    #[serde(rename = "0")]
    GuildText(TextChannel),
    #[serde(rename = "1")]
    DM(DMChannel),
    #[serde(rename = "2")]
    GuildVoice(VoiceChannel),
    #[serde(rename = "3")]
    GroupDM(GroupDMChannel),
    #[serde(rename = "4")]
    GuildCategory(CategoryChannel),
    #[serde(rename = "5")]
    GuildNews(NewsChannel),
    #[serde(rename = "10", rename_all = "snake_case")]
    GuildNewsThread(ThreadChannel),
    #[serde(rename = "11", rename_all = "snake_case")]
    GuildPublicThread(ThreadChannel),
    #[serde(rename = "12", rename_all = "snake_case")]
    GuildPrivateThread(ThreadChannel),
    #[serde(rename = "13")]
    GuildStageVoice(StageChannel),
    #[serde(rename = "15")]
    GuildForum(ForumChannel),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextChannel {
    pub id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub name: String,
    pub position: Option<u32>,
    pub permission_overwrites: Option<Vec<serde_json::Value>>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<Snowflake>,
    pub rate_limit_per_user: Option<u32>,
    pub parent_id: Option<Snowflake>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMChannel {
    pub id: Snowflake,
    pub last_message_id: Option<Snowflake>,
    pub recipients: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceChannel {
    pub id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub name: String,
    pub position: Option<u32>,
    pub permission_overwrites: Option<Vec<serde_json::Value>>,
    pub bitrate: Option<u32>,
    pub user_limit: Option<u32>,
    pub parent_id: Option<Snowflake>,
    pub rtc_region: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupDMChannel {
    pub id: Snowflake,
    pub name: Option<String>,
    pub recipients: Vec<serde_json::Value>,
    pub icon: Option<String>,
    pub owner_id: Snowflake,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryChannel {
    pub id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub name: String,
    pub position: Option<u32>,
    pub permission_overwrites: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsChannel {
    pub id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub name: String,
    pub position: Option<u32>,
    pub permission_overwrites: Option<Vec<serde_json::Value>>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<Snowflake>,
    pub parent_id: Option<Snowflake>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadChannel {
    pub id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub parent_id: Option<Snowflake>,
    pub owner_id: Option<Snowflake>,
    pub name: String,
    pub last_message_id: Option<Snowflake>,
    pub thread_metadata: Option<serde_json::Value>,
    pub message_count: Option<u32>,
    pub member_count: Option<u32>,
    pub rate_limit_per_user: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageChannel {
    pub id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub name: String,
    pub position: Option<u32>,
    pub permission_overwrites: Option<Vec<serde_json::Value>>,
    pub bitrate: Option<u32>,
    pub user_limit: Option<u32>,
    pub parent_id: Option<Snowflake>,
    pub rtc_region: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumChannel {
    pub id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub name: String,
    pub position: Option<u32>,
    pub permission_overwrites: Option<Vec<serde_json::Value>>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub parent_id: Option<Snowflake>,
    pub rate_limit_per_user: Option<u32>,
    pub available_tags: Option<Vec<serde_json::Value>>,
    pub default_reaction_emoji: Option<serde_json::Value>,
    pub default_thread_rate_limit_per_user: Option<u32>,
    pub default_sort_order: Option<u8>,
    pub default_forum_layout: Option<u8>,
}

impl Channel {
    pub fn id(&self) -> Snowflake {
        match self {
            Channel::GuildText(c) => c.id,
            Channel::DM(c) => c.id,
            Channel::GuildVoice(c) => c.id,
            Channel::GroupDM(c) => c.id,
            Channel::GuildCategory(c) => c.id,
            Channel::GuildNews(c) => c.id,
            Channel::GuildNewsThread(c) => c.id,
            Channel::GuildPublicThread(c) => c.id,
            Channel::GuildPrivateThread(c) => c.id,
            Channel::GuildStageVoice(c) => c.id,
            Channel::GuildForum(c) => c.id,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invite {
    pub code: String,
    pub channel: Option<serde_json::Value>,
    pub guild: Option<serde_json::Value>,
    pub inviter: Option<crate::models::user::User>,
    pub target_type: Option<u8>,
    pub target_user: Option<crate::models::user::User>,
    pub approximate_presence_count: Option<u32>,
    pub approximate_member_count: Option<u32>,
    pub expires_at: Option<String>,
    pub uses: Option<u32>,
    pub max_uses: Option<u32>,
    pub max_age: Option<u32>,
    pub temporary: Option<bool>,
    pub created_at: Option<String>,
}

impl Channel {
    /// Returns the channel name if available.
    pub fn name(&self) -> Option<&str> {
        match self {
            Channel::GuildText(c) => Some(&c.name),
            Channel::GuildVoice(c) => Some(&c.name),
            Channel::GuildCategory(c) => Some(&c.name),
            Channel::GuildNews(c) => Some(&c.name),
            Channel::GuildNewsThread(c) => Some(&c.name),
            Channel::GuildPublicThread(c) => Some(&c.name),
            Channel::GuildPrivateThread(c) => Some(&c.name),
            Channel::GuildStageVoice(c) => Some(&c.name),
            Channel::GuildForum(c) => Some(&c.name),
            Channel::GroupDM(c) => c.name.as_deref(),
            Channel::DM(_) => None,
        }
    }

    /// Returns the guild ID if this channel belongs to a guild.
    pub fn guild_id(&self) -> Option<Snowflake> {
        match self {
            Channel::GuildText(c) => c.guild_id,
            Channel::GuildVoice(c) => c.guild_id,
            Channel::GuildCategory(c) => c.guild_id,
            Channel::GuildNews(c) => c.guild_id,
            Channel::GuildNewsThread(c) => c.guild_id,
            Channel::GuildPublicThread(c) => c.guild_id,
            Channel::GuildPrivateThread(c) => c.guild_id,
            Channel::GuildStageVoice(c) => c.guild_id,
            Channel::GuildForum(c) => c.guild_id,
            _ => None,
        }
    }

    /// Returns true if this is a DM or group DM channel.
    pub fn is_dm(&self) -> bool {
        matches!(self, Channel::DM(_) | Channel::GroupDM(_))
    }

    /// Returns true if this is a thread channel.
    pub fn is_thread(&self) -> bool {
        matches!(
            self,
            Channel::GuildNewsThread(_) | Channel::GuildPublicThread(_) | Channel::GuildPrivateThread(_)
        )
    }

    /// Returns the channel mention string.
    pub fn mention(&self) -> String {
        format!("<#{}>", self.id())
    }
}
