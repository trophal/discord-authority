use crate::models::{Message, User, Guild, Channel, Presence};
use crate::models::guild::{GuildMember, Role};
use crate::models::channel::Invite;
use async_trait::async_trait;

#[async_trait]
pub trait EventHandler: Send + Sync {
    // ── Connection ──────────────────────────────────────────────────────────

    /// Fired when the client has authenticated and is ready to receive events.
    async fn on_ready(&self, _user: User) {}

    /// Fired when the gateway connection is resumed after a disconnect.
    async fn on_resumed(&self) {}

    // ── Messages ─────────────────────────────────────────────────────────────

    /// Fired when a new message is sent in any channel the client can see.
    async fn on_message(&self, _message: Message) {}

    /// Fired when a message is edited. `old` is `None` if not cached.
    async fn on_message_edit(&self, _old: Option<Message>, _new: Message) {}

    /// Fired when a message is deleted.
    async fn on_message_delete(&self, _channel_id: crate::utils::Snowflake, _message_id: crate::utils::Snowflake) {}

    /// Fired when multiple messages are bulk-deleted.
    async fn on_message_bulk_delete(&self, _channel_id: crate::utils::Snowflake, _message_ids: Vec<crate::utils::Snowflake>) {}

    // ── Reactions ────────────────────────────────────────────────────────────

    /// Fired when a reaction is added to a message.
    async fn on_reaction_add(&self, _reaction: ReactionEvent) {}

    /// Fired when a reaction is removed from a message.
    async fn on_reaction_remove(&self, _reaction: ReactionEvent) {}

    /// Fired when all reactions are cleared from a message.
    async fn on_reaction_clear(&self, _channel_id: crate::utils::Snowflake, _message_id: crate::utils::Snowflake) {}

    /// Fired when all reactions for a specific emoji are removed.
    async fn on_reaction_clear_emoji(&self, _channel_id: crate::utils::Snowflake, _message_id: crate::utils::Snowflake, _emoji: String) {}

    // ── Guilds ───────────────────────────────────────────────────────────────

    /// Fired when the client joins a guild or a guild becomes available.
    async fn on_guild_join(&self, _guild: Guild) {}

    /// Fired when a guild is updated (name, icon, etc. changed).
    async fn on_guild_update(&self, _old: Option<Guild>, _new: Guild) {}

    /// Fired when the client leaves a guild or it becomes unavailable.
    async fn on_guild_leave(&self, _guild_id: crate::utils::Snowflake) {}

    /// Fired when a guild becomes unavailable due to an outage.
    async fn on_guild_unavailable(&self, _guild_id: crate::utils::Snowflake) {}

    // ── Members ──────────────────────────────────────────────────────────────

    /// Fired when a user joins a guild.
    async fn on_member_join(&self, _guild_id: crate::utils::Snowflake, _member: GuildMember) {}

    /// Fired when a guild member is updated (roles, nickname, etc.).
    async fn on_member_update(&self, _guild_id: crate::utils::Snowflake, _member: GuildMember) {}

    /// Fired when a user leaves or is removed from a guild.
    async fn on_member_leave(&self, _guild_id: crate::utils::Snowflake, _user: User) {}

    // ── Roles ────────────────────────────────────────────────────────────────

    /// Fired when a role is created in a guild.
    async fn on_role_create(&self, _guild_id: crate::utils::Snowflake, _role: Role) {}

    /// Fired when a role is updated.
    async fn on_role_update(&self, _guild_id: crate::utils::Snowflake, _role: Role) {}

    /// Fired when a role is deleted.
    async fn on_role_delete(&self, _guild_id: crate::utils::Snowflake, _role_id: crate::utils::Snowflake) {}

    // ── Channels ─────────────────────────────────────────────────────────────

    /// Fired when a channel is created.
    async fn on_channel_create(&self, _channel: Channel) {}

    /// Fired when a channel is updated.
    async fn on_channel_update(&self, _old: Option<Channel>, _new: Channel) {}

    /// Fired when a channel is deleted.
    async fn on_channel_delete(&self, _channel: Channel) {}

    /// Fired when a channel's pinned messages are updated.
    async fn on_channel_pins_update(&self, _channel_id: crate::utils::Snowflake, _last_pin_timestamp: Option<String>) {}

    // ── Threads ──────────────────────────────────────────────────────────────

    /// Fired when a thread is created or the client is added to a private thread.
    async fn on_thread_create(&self, _channel: Channel) {}

    /// Fired when a thread is updated.
    async fn on_thread_update(&self, _channel: Channel) {}

    /// Fired when a thread is deleted or the client is removed from a private thread.
    async fn on_thread_delete(&self, _channel_id: crate::utils::Snowflake, _guild_id: Option<crate::utils::Snowflake>) {}

    // ── Invites ──────────────────────────────────────────────────────────────

    /// Fired when an invite is created.
    async fn on_invite_create(&self, _invite: Invite) {}

    /// Fired when an invite is deleted.
    async fn on_invite_delete(&self, _channel_id: crate::utils::Snowflake, _code: String) {}

    // ── Bans ─────────────────────────────────────────────────────────────────

    /// Fired when a user is banned from a guild.
    async fn on_ban_add(&self, _guild_id: crate::utils::Snowflake, _user: User) {}

    /// Fired when a user's ban is removed.
    async fn on_ban_remove(&self, _guild_id: crate::utils::Snowflake, _user: User) {}

    // ── Presence & Typing ────────────────────────────────────────────────────

    /// Fired when a user's presence (status/activity) changes.
    async fn on_presence_update(&self, _presence: Presence) {}

    /// Fired when a user starts typing in a channel.
    async fn on_typing_start(&self, _channel_id: crate::utils::Snowflake, _user_id: crate::utils::Snowflake) {}

    // ── Polls ────────────────────────────────────────────────────────────────

    /// Fired when a vote is added to a poll.
    async fn on_poll_vote_add(&self, _user_id: crate::utils::Snowflake, _channel_id: crate::utils::Snowflake, _message_id: crate::utils::Snowflake, _answer_id: u32) {}

    /// Fired when a vote is removed from a poll.
    async fn on_poll_vote_remove(&self, _user_id: crate::utils::Snowflake, _channel_id: crate::utils::Snowflake, _message_id: crate::utils::Snowflake, _answer_id: u32) {}

    // ── Users ────────────────────────────────────────────────────────────────

    /// Fired when the current user's settings are updated.
    async fn on_user_update(&self, _user: User) {}

    // ── Raw ──────────────────────────────────────────────────────────────────

    /// Fired for every raw gateway event. Useful for debugging or handling unsupported events.
    async fn on_raw_event(&self, _event: serde_json::Value) {}
}

/// Data for a reaction add/remove event.
#[derive(Debug, Clone)]
pub struct ReactionEvent {
    pub user_id: crate::utils::Snowflake,
    pub channel_id: crate::utils::Snowflake,
    pub message_id: crate::utils::Snowflake,
    pub guild_id: Option<crate::utils::Snowflake>,
    pub emoji: String,
}

/// Default no-op event handler.
pub struct DefaultEventHandler;

#[async_trait]
impl EventHandler for DefaultEventHandler {}
