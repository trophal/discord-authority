use crate::models::{Message, User, Guild, Channel, Presence};
use async_trait::async_trait;

#[async_trait]
pub trait EventHandler: Send + Sync {
    /// Called when the client is ready
    async fn ready(&self, user: User) {}

    /// Called when a message is created
    async fn message_create(&self, message: Message) {}

    /// Called when a message is updated
    async fn message_update(&self, old_message: Option<Message>, new_message: Message) {}

    /// Called when a message is deleted
    async fn message_delete(&self, channel_id: crate::utils::Snowflake, message_id: crate::utils::Snowflake) {}

    /// Called when a guild is created (bot joined)
    async fn guild_create(&self, guild: Guild) {}

    /// Called when a guild is deleted (bot left)
    async fn guild_delete(&self, guild_id: crate::utils::Snowflake) {}

    /// Called when a channel is created
    async fn channel_create(&self, channel: Channel) {}

    /// Called when a channel is deleted
    async fn channel_delete(&self, channel: Channel) {}

    /// Called when a presence is updated
    async fn presence_update(&self, presence: Presence) {}

    /// Called when typing starts
    async fn typing_start(&self, channel_id: crate::utils::Snowflake, user_id: crate::utils::Snowflake) {}

    /// Called when a poll vote is added
    async fn poll_vote_add(&self, user_id: crate::utils::Snowflake, channel_id: crate::utils::Snowflake, message_id: crate::utils::Snowflake, answer_id: u32) {}

    /// Called when a poll vote is removed
    async fn poll_vote_remove(&self, user_id: crate::utils::Snowflake, channel_id: crate::utils::Snowflake, message_id: crate::utils::Snowflake, answer_id: u32) {}

    /// Called on any raw event (useful for debugging)
    async fn raw(&self, _event: serde_json::Value) {}
}

/// Default event handler that does nothing
pub struct DefaultEventHandler;

#[async_trait]
impl EventHandler for DefaultEventHandler {}

