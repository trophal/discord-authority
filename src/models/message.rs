use serde::{Deserialize, Serialize};
use crate::utils::Snowflake;
use super::user::User;
use super::embed::Embed;
use super::poll::Poll;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    pub author: User,
    pub content: String,
    pub timestamp: String,
    pub edited_timestamp: Option<String>,
    pub tts: bool,
    pub mention_everyone: bool,
    pub mentions: Vec<User>,
    pub mention_roles: Vec<Snowflake>,
    pub attachments: Vec<Attachment>,
    pub embeds: Vec<Embed>,
    pub reactions: Option<Vec<Reaction>>,
    pub nonce: Option<serde_json::Value>,
    pub pinned: bool,
    pub webhook_id: Option<Snowflake>,
    #[serde(rename = "type")]
    pub message_type: u8,
    pub activity: Option<MessageActivity>,
    pub application: Option<serde_json::Value>,
    pub application_id: Option<Snowflake>,
    pub message_reference: Option<MessageReference>,
    pub flags: Option<u32>,
    pub referenced_message: Option<Box<Message>>,
    pub interaction: Option<serde_json::Value>,
    pub thread: Option<serde_json::Value>,
    pub components: Option<Vec<serde_json::Value>>,
    pub sticker_items: Option<Vec<serde_json::Value>>,
    pub poll: Option<Poll>,
    pub guild_id: Option<Snowflake>,
    pub member: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub id: Snowflake,
    pub filename: String,
    pub description: Option<String>,
    pub content_type: Option<String>,
    pub size: u64,
    pub url: String,
    pub proxy_url: String,
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub ephemeral: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reaction {
    pub count: u32,
    pub me: bool,
    pub emoji: ReactionEmoji,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionEmoji {
    pub id: Option<Snowflake>,
    pub name: Option<String>,
    pub animated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageActivity {
    #[serde(rename = "type")]
    pub activity_type: u8,
    pub party_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageReference {
    pub message_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub guild_id: Option<Snowflake>,
    pub fail_if_not_exists: Option<bool>,
}

#[derive(Debug, Default, Clone)]
pub struct MessageBuilder {
    content: Option<String>,
    embeds: Vec<Embed>,
    tts: bool,
    nonce: Option<String>,
    reply_to: Option<Snowflake>,
    poll: Option<Poll>,
    activity: Option<MessageActivity>,
}

impl MessageBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn content<S: Into<String>>(mut self, content: S) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn embed(mut self, embed: Embed) -> Self {
        self.embeds.push(embed);
        self
    }

    pub fn embeds(mut self, embeds: Vec<Embed>) -> Self {
        self.embeds = embeds;
        self
    }

    pub fn tts(mut self, tts: bool) -> Self {
        self.tts = tts;
        self
    }

    pub fn reply_to(mut self, message_id: Snowflake) -> Self {
        self.reply_to = Some(message_id);
        self
    }

    pub fn poll(mut self, poll: Poll) -> Self {
        self.poll = Some(poll);
        self
    }

    pub fn activity(mut self, activity: MessageActivity) -> Self {
        self.activity = Some(activity);
        self
    }

    pub fn build(self) -> serde_json::Value {
        let mut json = serde_json::json!({
            "tts": self.tts,
        });

        if let Some(content) = self.content {
            json["content"] = serde_json::json!(content);
        }

        if !self.embeds.is_empty() {
            json["embeds"] = serde_json::json!(self.embeds);
        }

        if let Some(nonce) = self.nonce {
            json["nonce"] = serde_json::json!(nonce);
        }

        if let Some(reply_to) = self.reply_to {
            json["message_reference"] = serde_json::json!({
                "message_id": reply_to.to_string(),
            });
        }

        if let Some(poll) = self.poll {
            json["poll"] = serde_json::to_value(poll).unwrap();
        }

        if let Some(activity) = self.activity {
            json["activity"] = serde_json::to_value(activity).unwrap();
        }

        json
    }
}

