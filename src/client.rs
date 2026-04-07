use crate::{Error, Result};
use crate::events::{EventHandler, DefaultEventHandler};
use crate::gateway::Gateway;
use crate::http::HttpClient;
use crate::models::{User, Message, Activity, Channel, Guild};
use crate::models::guild::{GuildMember, Role, Ban};
use crate::models::channel::Invite;
use crate::models::message::MessageBuilder;
use crate::models::embed::Embed;
use crate::utils::Snowflake;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

pub struct Client {
    gateway: Arc<Gateway>,
    pub http: Arc<HttpClient>,
    user: Arc<RwLock<Option<User>>>,
}

impl Client {
    // ── Self ──────────────────────────────────────────────────────────────────

    /// Returns the current authenticated user.
    pub async fn current_user(&self) -> Option<User> {
        self.user.read().await.clone()
    }

    /// Refreshes and returns the current user from the API.
    pub async fn fetch_me(&self) -> Result<User> {
        self.http.get_me().await
    }

    // ── Messages ──────────────────────────────────────────────────────────────

    /// Send a plain text message to a channel.
    pub async fn send_message(&self, channel_id: Snowflake, content: impl Into<String>) -> Result<Message> {
        self.http.send_message(channel_id, serde_json::json!({ "content": content.into() })).await
    }

    /// Send a message using a `MessageBuilder` payload.
    pub async fn send(&self, channel_id: Snowflake, builder: MessageBuilder) -> Result<Message> {
        self.http.send_message(channel_id, builder.build()).await
    }

    /// Send an embed to a channel.
    pub async fn send_embed(&self, channel_id: Snowflake, embed: Embed) -> Result<Message> {
        let payload = MessageBuilder::new().embed(embed).build();
        self.http.send_message(channel_id, payload).await
    }

    /// Send a message with content and an embed.
    pub async fn send_message_with_embed(
        &self,
        channel_id: Snowflake,
        content: impl Into<String>,
        embed: Embed,
    ) -> Result<Message> {
        let payload = MessageBuilder::new().content(content.into()).embed(embed).build();
        self.http.send_message(channel_id, payload).await
    }

    /// Edit a message's content.
    pub async fn edit_message(
        &self,
        channel_id: Snowflake,
        message_id: Snowflake,
        content: impl Into<String>,
    ) -> Result<Message> {
        self.http.edit_message(channel_id, message_id, serde_json::json!({ "content": content.into() })).await
    }

    /// Delete a message.
    pub async fn delete_message(&self, channel_id: Snowflake, message_id: Snowflake) -> Result<()> {
        self.http.delete_message(channel_id, message_id).await
    }

    /// Bulk delete messages (2–100, must be under 14 days old).
    pub async fn bulk_delete_messages(&self, channel_id: Snowflake, message_ids: Vec<Snowflake>) -> Result<()> {
        self.http.bulk_delete_messages(channel_id, message_ids).await
    }

    /// Fetch a single message.
    pub async fn get_message(&self, channel_id: Snowflake, message_id: Snowflake) -> Result<Message> {
        self.http.get_message(channel_id, message_id).await
    }

    /// Fetch message history from a channel.
    pub async fn get_messages(
        &self,
        channel_id: Snowflake,
        limit: u8,
        before: Option<Snowflake>,
        after: Option<Snowflake>,
    ) -> Result<Vec<Message>> {
        self.http.get_messages(channel_id, limit, before, after, None).await
    }

    /// Pin a message in a channel.
    pub async fn pin_message(&self, channel_id: Snowflake, message_id: Snowflake) -> Result<()> {
        self.http.pin_message(channel_id, message_id).await
    }

    /// Unpin a message from a channel.
    pub async fn unpin_message(&self, channel_id: Snowflake, message_id: Snowflake) -> Result<()> {
        self.http.unpin_message(channel_id, message_id).await
    }

    /// Fetch pinned messages in a channel.
    pub async fn get_pinned_messages(&self, channel_id: Snowflake) -> Result<Vec<Message>> {
        self.http.get_pinned_messages(channel_id).await
    }

    // ── Reactions ─────────────────────────────────────────────────────────────

    /// Add a reaction to a message.
    pub async fn add_reaction(&self, channel_id: Snowflake, message_id: Snowflake, emoji: &str) -> Result<()> {
        self.http.add_reaction(channel_id, message_id, emoji).await
    }

    /// Remove your reaction from a message.
    pub async fn remove_reaction(&self, channel_id: Snowflake, message_id: Snowflake, emoji: &str) -> Result<()> {
        self.http.remove_reaction(channel_id, message_id, emoji).await
    }

    /// Remove all reactions from a message.
    pub async fn clear_reactions(&self, channel_id: Snowflake, message_id: Snowflake) -> Result<()> {
        self.http.clear_reactions(channel_id, message_id).await
    }

    // ── Channels ──────────────────────────────────────────────────────────────

    /// Fetch a channel by ID.
    pub async fn get_channel(&self, channel_id: Snowflake) -> Result<Channel> {
        self.http.get_channel(channel_id).await
    }

    /// Open or fetch a DM channel with a user.
    pub async fn create_dm(&self, user_id: Snowflake) -> Result<Channel> {
        self.http.create_dm(user_id).await
    }

    /// Send a DM to a user (opens the DM channel automatically).
    pub async fn send_dm(&self, user_id: Snowflake, content: impl Into<String>) -> Result<Message> {
        let channel = self.create_dm(user_id).await?;
        self.send_message(channel.id(), content).await
    }

    /// Start the typing indicator in a channel.
    pub async fn start_typing(&self, channel_id: Snowflake) -> Result<()> {
        self.http.start_typing(channel_id).await
    }

    /// Create an invite for a channel.
    pub async fn create_invite(
        &self,
        channel_id: Snowflake,
        max_age: u32,
        max_uses: u32,
        temporary: bool,
    ) -> Result<Invite> {
        self.http.create_invite(channel_id, max_age, max_uses, temporary).await
    }

    // ── Polls ─────────────────────────────────────────────────────────────────

    /// Vote on a poll answer.
    pub async fn vote_poll(&self, channel_id: Snowflake, message_id: Snowflake, answer_id: u32) -> Result<()> {
        self.http.vote_poll(channel_id, message_id, answer_id).await
    }

    /// Remove your vote from a poll answer.
    pub async fn unvote_poll(&self, channel_id: Snowflake, message_id: Snowflake, answer_id: u32) -> Result<()> {
        self.http.unvote_poll(channel_id, message_id, answer_id).await
    }

    // ── Guilds ────────────────────────────────────────────────────────────────

    /// Fetch a guild by ID.
    pub async fn get_guild(&self, guild_id: Snowflake) -> Result<Guild> {
        self.http.get_guild(guild_id).await
    }

    /// Fetch all guilds the current user is in.
    pub async fn get_my_guilds(&self) -> Result<Vec<serde_json::Value>> {
        self.http.get_my_guilds().await
    }

    /// Leave a guild.
    pub async fn leave_guild(&self, guild_id: Snowflake) -> Result<()> {
        self.http.leave_guild(guild_id).await
    }

    // ── Members ───────────────────────────────────────────────────────────────

    /// Fetch a guild member.
    pub async fn get_member(&self, guild_id: Snowflake, user_id: Snowflake) -> Result<GuildMember> {
        self.http.get_member(guild_id, user_id).await
    }

    /// Kick a member from a guild.
    pub async fn kick_member(&self, guild_id: Snowflake, user_id: Snowflake) -> Result<()> {
        self.http.kick_member(guild_id, user_id).await
    }

    /// Ban a member from a guild.
    pub async fn ban_member(
        &self,
        guild_id: Snowflake,
        user_id: Snowflake,
        delete_message_seconds: u32,
        reason: Option<&str>,
    ) -> Result<()> {
        self.http.ban_member(guild_id, user_id, delete_message_seconds, reason).await
    }

    /// Unban a user from a guild.
    pub async fn unban_member(&self, guild_id: Snowflake, user_id: Snowflake) -> Result<()> {
        self.http.unban_member(guild_id, user_id).await
    }

    /// Fetch the ban list for a guild.
    pub async fn get_bans(&self, guild_id: Snowflake) -> Result<Vec<Ban>> {
        self.http.get_bans(guild_id).await
    }

    /// Add a role to a guild member.
    pub async fn add_role(&self, guild_id: Snowflake, user_id: Snowflake, role_id: Snowflake) -> Result<()> {
        self.http.add_member_role(guild_id, user_id, role_id).await
    }

    /// Remove a role from a guild member.
    pub async fn remove_role(&self, guild_id: Snowflake, user_id: Snowflake, role_id: Snowflake) -> Result<()> {
        self.http.remove_member_role(guild_id, user_id, role_id).await
    }

    /// Set a member's nickname. Pass `None` to reset it.
    pub async fn set_nickname(&self, guild_id: Snowflake, user_id: Snowflake, nick: Option<&str>) -> Result<()> {
        self.http.set_nickname(guild_id, user_id, nick).await
    }

    // ── Roles ─────────────────────────────────────────────────────────────────

    /// Fetch all roles in a guild.
    pub async fn get_roles(&self, guild_id: Snowflake) -> Result<Vec<Role>> {
        self.http.get_roles(guild_id).await
    }

    /// Delete a role from a guild.
    pub async fn delete_role(&self, guild_id: Snowflake, role_id: Snowflake) -> Result<()> {
        self.http.delete_role(guild_id, role_id).await
    }

    // ── Users ─────────────────────────────────────────────────────────────────

    /// Fetch a user by ID.
    pub async fn get_user(&self, user_id: Snowflake) -> Result<User> {
        self.http.get_user(user_id).await
    }

    // ── Presence ──────────────────────────────────────────────────────────────

    /// Set presence with multiple activities and a status string.
    pub async fn set_presence(&self, activities: Vec<Activity>, status: &str) -> Result<()> {
        self.gateway.update_presence(activities, status).await
    }

    /// Set a single activity (keeps status as "online").
    pub async fn set_activity(&self, activity: Activity) -> Result<()> {
        self.gateway.update_presence(vec![activity], "online").await
    }

    /// Set status only (online, idle, dnd, invisible).
    pub async fn set_status(&self, status: &str) -> Result<()> {
        self.gateway.update_presence(vec![], status).await
    }

    /// Clear all activities.
    pub async fn clear_activity(&self) -> Result<()> {
        self.gateway.update_presence(vec![], "online").await
    }

    // ── Gateway ───────────────────────────────────────────────────────────────

    /// Start listening for gateway events. This blocks until the connection closes.
    pub async fn listen(&self) -> Result<()> {
        self.gateway.listen().await
    }
}

pub struct ClientBuilder {
    token: String,
    event_handler: Option<Arc<dyn EventHandler>>,
}

impl ClientBuilder {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            event_handler: None,
        }
    }

    pub fn event_handler(mut self, handler: Arc<dyn EventHandler>) -> Self {
        self.event_handler = Some(handler);
        self
    }

    pub async fn build(self) -> Result<Client> {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", "info");
        }
        tracing_subscriber::fmt::init();

        let token = self.token;
        let event_handler = self.event_handler.unwrap_or_else(|| Arc::new(DefaultEventHandler));

        let http = Arc::new(HttpClient::new(token.clone()));

        info!("Verifying token...");
        let user = http.get_me().await.map_err(|_| Error::InvalidToken)?;
        info!("Authenticated as: {}", user.tag());

        let gateway = Arc::new(Gateway::new(token, event_handler));

        info!("Connecting to Discord Gateway...");
        gateway.connect().await?;

        Ok(Client {
            gateway,
            http,
            user: Arc::new(RwLock::new(Some(user))),
        })
    }
}

// ── Message helper methods ────────────────────────────────────────────────────

impl Message {
    /// Reply to this message.
    pub async fn reply(&self, http: &HttpClient, content: impl Into<String>) -> Result<Message> {
        let payload = serde_json::json!({
            "content": content.into(),
            "message_reference": {
                "message_id": self.id.to_string(),
                "channel_id": self.channel_id.to_string(),
            }
        });
        http.send_message(self.channel_id, payload).await
    }

    /// Reply with an embed.
    pub async fn reply_embed(&self, http: &HttpClient, embed: Embed) -> Result<Message> {
        let payload = MessageBuilder::new()
            .embed(embed)
            .reply_to(self.id)
            .build();
        http.send_message(self.channel_id, payload).await
    }

    /// Add a reaction to this message.
    pub async fn react(&self, http: &HttpClient, emoji: &str) -> Result<()> {
        http.add_reaction(self.channel_id, self.id, emoji).await
    }

    /// Delete this message.
    pub async fn delete(&self, http: &HttpClient) -> Result<()> {
        http.delete_message(self.channel_id, self.id).await
    }

    /// Edit this message's content.
    pub async fn edit(&self, http: &HttpClient, content: impl Into<String>) -> Result<Message> {
        http.edit_message(self.channel_id, self.id, serde_json::json!({ "content": content.into() })).await
    }

    /// Pin this message.
    pub async fn pin(&self, http: &HttpClient) -> Result<()> {
        http.pin_message(self.channel_id, self.id).await
    }

    /// Unpin this message.
    pub async fn unpin(&self, http: &HttpClient) -> Result<()> {
        http.unpin_message(self.channel_id, self.id).await
    }

    /// Returns true if this message was sent by a bot.
    pub fn is_from_bot(&self) -> bool {
        self.author.bot
    }

    /// Returns true if this message mentions everyone.
    pub fn mentions_everyone(&self) -> bool {
        self.mention_everyone
    }

    /// Returns true if this message has any embeds.
    pub fn has_embeds(&self) -> bool {
        !self.embeds.is_empty()
    }

    /// Returns true if this message has any attachments.
    pub fn has_attachments(&self) -> bool {
        !self.attachments.is_empty()
    }
}
