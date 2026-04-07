use crate::{Error, Result};
use crate::utils::Snowflake;
use crate::models::{Message, Channel, User};
use crate::models::guild::{Guild, GuildMember, Role, Ban};
use crate::models::channel::Invite;
use reqwest::{Client as ReqwestClient, header};
use serde_json::json;
use tracing::debug;

pub struct HttpClient {
    client: ReqwestClient,
    base_url: String,
}

impl HttpClient {
    pub fn new(token: String) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&token).unwrap(),
        );
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            ),
        );

        let client = ReqwestClient::builder()
            .default_headers(headers)
            .cookie_store(true)
            .build()
            .unwrap();

        Self {
            client,
            base_url: "https://discord.com/api/v9".to_string(),
        }
    }

    // ── Internal helpers ─────────────────────────────────────────────────────

    async fn check_response(&self, response: reqwest::Response) -> Result<reqwest::Response> {
        if response.status().is_success() {
            Ok(response)
        } else {
            let status = response.status();
            let text = response.text().await?;
            Err(Error::DiscordApi {
                code: status.as_u16() as u64,
                message: text,
            })
        }
    }

    // ── Users ─────────────────────────────────────────────────────────────────

    /// Fetch the current authenticated user.
    pub async fn get_me(&self) -> Result<User> {
        let url = format!("{}/users/@me", self.base_url);
        let res = self.check_response(self.client.get(&url).send().await?).await?;
        Ok(res.json().await?)
    }

    /// Fetch a user by ID.
    pub async fn get_user(&self, user_id: Snowflake) -> Result<User> {
        let url = format!("{}/users/{}", self.base_url, user_id);
        let res = self.check_response(self.client.get(&url).send().await?).await?;
        Ok(res.json().await?)
    }

    /// Open or fetch an existing DM channel with a user.
    pub async fn create_dm(&self, user_id: Snowflake) -> Result<Channel> {
        let url = format!("{}/users/@me/channels", self.base_url);
        let res = self.check_response(
            self.client.post(&url).json(&json!({ "recipient_id": user_id.to_string() })).send().await?,
        ).await?;
        Ok(res.json().await?)
    }

    // ── Channels ──────────────────────────────────────────────────────────────

    /// Fetch a channel by ID.
    pub async fn get_channel(&self, channel_id: Snowflake) -> Result<Channel> {
        let url = format!("{}/channels/{}", self.base_url, channel_id);
        let res = self.check_response(self.client.get(&url).send().await?).await?;
        Ok(res.json().await?)
    }

    /// Delete (or close) a channel.
    pub async fn delete_channel(&self, channel_id: Snowflake) -> Result<()> {
        let url = format!("{}/channels/{}", self.base_url, channel_id);
        self.check_response(self.client.delete(&url).send().await?).await?;
        Ok(())
    }

    /// Trigger the typing indicator in a channel.
    pub async fn start_typing(&self, channel_id: Snowflake) -> Result<()> {
        let url = format!("{}/channels/{}/typing", self.base_url, channel_id);
        self.check_response(self.client.post(&url).send().await?).await?;
        Ok(())
    }

    /// Fetch pinned messages in a channel.
    pub async fn get_pinned_messages(&self, channel_id: Snowflake) -> Result<Vec<Message>> {
        let url = format!("{}/channels/{}/pins", self.base_url, channel_id);
        let res = self.check_response(self.client.get(&url).send().await?).await?;
        Ok(res.json().await?)
    }

    /// Pin a message in a channel.
    pub async fn pin_message(&self, channel_id: Snowflake, message_id: Snowflake) -> Result<()> {
        let url = format!("{}/channels/{}/pins/{}", self.base_url, channel_id, message_id);
        self.check_response(self.client.put(&url).send().await?).await?;
        Ok(())
    }

    /// Unpin a message from a channel.
    pub async fn unpin_message(&self, channel_id: Snowflake, message_id: Snowflake) -> Result<()> {
        let url = format!("{}/channels/{}/pins/{}", self.base_url, channel_id, message_id);
        self.check_response(self.client.delete(&url).send().await?).await?;
        Ok(())
    }

    /// Create an invite for a channel.
    pub async fn create_invite(
        &self,
        channel_id: Snowflake,
        max_age: u32,
        max_uses: u32,
        temporary: bool,
    ) -> Result<Invite> {
        let url = format!("{}/channels/{}/invites", self.base_url, channel_id);
        let res = self.check_response(
            self.client.post(&url).json(&json!({
                "max_age": max_age,
                "max_uses": max_uses,
                "temporary": temporary,
            })).send().await?,
        ).await?;
        Ok(res.json().await?)
    }

    /// Fetch all invites for a channel.
    pub async fn get_channel_invites(&self, channel_id: Snowflake) -> Result<Vec<Invite>> {
        let url = format!("{}/channels/{}/invites", self.base_url, channel_id);
        let res = self.check_response(self.client.get(&url).send().await?).await?;
        Ok(res.json().await?)
    }

    // ── Messages ──────────────────────────────────────────────────────────────

    /// Fetch a single message.
    pub async fn get_message(&self, channel_id: Snowflake, message_id: Snowflake) -> Result<Message> {
        let url = format!("{}/channels/{}/messages/{}", self.base_url, channel_id, message_id);
        let res = self.check_response(self.client.get(&url).send().await?).await?;
        Ok(res.json().await?)
    }

    /// Fetch message history. `before`/`after`/`around` are optional snowflake anchors.
    pub async fn get_messages(
        &self,
        channel_id: Snowflake,
        limit: u8,
        before: Option<Snowflake>,
        after: Option<Snowflake>,
        around: Option<Snowflake>,
    ) -> Result<Vec<Message>> {
        let mut url = format!("{}/channels/{}/messages?limit={}", self.base_url, channel_id, limit.min(100));
        if let Some(b) = before { url.push_str(&format!("&before={}", b)); }
        if let Some(a) = after  { url.push_str(&format!("&after={}", a)); }
        if let Some(a) = around { url.push_str(&format!("&around={}", a)); }
        let res = self.check_response(self.client.get(&url).send().await?).await?;
        Ok(res.json().await?)
    }

    /// Send a message with a raw JSON payload.
    pub async fn send_message(&self, channel_id: Snowflake, payload: serde_json::Value) -> Result<Message> {
        let url = format!("{}/channels/{}/messages", self.base_url, channel_id);
        debug!("Sending message to channel {}", channel_id);
        let res = self.check_response(self.client.post(&url).json(&payload).send().await?).await?;
        Ok(res.json().await?)
    }

    /// Edit a message with a raw JSON payload.
    pub async fn edit_message(
        &self,
        channel_id: Snowflake,
        message_id: Snowflake,
        payload: serde_json::Value,
    ) -> Result<Message> {
        let url = format!("{}/channels/{}/messages/{}", self.base_url, channel_id, message_id);
        let res = self.check_response(self.client.patch(&url).json(&payload).send().await?).await?;
        Ok(res.json().await?)
    }

    /// Delete a message.
    pub async fn delete_message(&self, channel_id: Snowflake, message_id: Snowflake) -> Result<()> {
        let url = format!("{}/channels/{}/messages/{}", self.base_url, channel_id, message_id);
        self.check_response(self.client.delete(&url).send().await?).await?;
        Ok(())
    }

    /// Bulk delete messages (2–100 messages, must be under 14 days old).
    pub async fn bulk_delete_messages(&self, channel_id: Snowflake, message_ids: Vec<Snowflake>) -> Result<()> {
        let url = format!("{}/channels/{}/messages/bulk-delete", self.base_url, channel_id);
        let ids: Vec<String> = message_ids.iter().map(|id| id.to_string()).collect();
        self.check_response(
            self.client.post(&url).json(&json!({ "messages": ids })).send().await?,
        ).await?;
        Ok(())
    }

    // ── Reactions ─────────────────────────────────────────────────────────────

    /// Add a reaction to a message.
    pub async fn add_reaction(&self, channel_id: Snowflake, message_id: Snowflake, emoji: &str) -> Result<()> {
        let url = format!(
            "{}/channels/{}/messages/{}/reactions/{}/@me",
            self.base_url, channel_id, message_id, emoji
        );
        self.check_response(self.client.put(&url).send().await?).await?;
        Ok(())
    }

    /// Remove the current user's reaction from a message.
    pub async fn remove_reaction(&self, channel_id: Snowflake, message_id: Snowflake, emoji: &str) -> Result<()> {
        let url = format!(
            "{}/channels/{}/messages/{}/reactions/{}/@me",
            self.base_url, channel_id, message_id, emoji
        );
        self.check_response(self.client.delete(&url).send().await?).await?;
        Ok(())
    }

    /// Remove another user's reaction from a message.
    pub async fn remove_user_reaction(
        &self,
        channel_id: Snowflake,
        message_id: Snowflake,
        emoji: &str,
        user_id: Snowflake,
    ) -> Result<()> {
        let url = format!(
            "{}/channels/{}/messages/{}/reactions/{}/{}",
            self.base_url, channel_id, message_id, emoji, user_id
        );
        self.check_response(self.client.delete(&url).send().await?).await?;
        Ok(())
    }

    /// Remove all reactions from a message.
    pub async fn clear_reactions(&self, channel_id: Snowflake, message_id: Snowflake) -> Result<()> {
        let url = format!(
            "{}/channels/{}/messages/{}/reactions",
            self.base_url, channel_id, message_id
        );
        self.check_response(self.client.delete(&url).send().await?).await?;
        Ok(())
    }

    // ── Polls ─────────────────────────────────────────────────────────────────

    /// Vote on a poll answer.
    pub async fn vote_poll(&self, channel_id: Snowflake, message_id: Snowflake, answer_id: u32) -> Result<()> {
        let url = format!(
            "{}/channels/{}/polls/{}/answers/{}",
            self.base_url, channel_id, message_id, answer_id
        );
        self.check_response(self.client.put(&url).send().await?).await?;
        Ok(())
    }

    /// Remove your vote from a poll answer.
    pub async fn unvote_poll(&self, channel_id: Snowflake, message_id: Snowflake, answer_id: u32) -> Result<()> {
        let url = format!(
            "{}/channels/{}/polls/{}/answers/{}",
            self.base_url, channel_id, message_id, answer_id
        );
        self.check_response(self.client.delete(&url).send().await?).await?;
        Ok(())
    }

    // ── Guilds ────────────────────────────────────────────────────────────────

    /// Fetch a guild by ID.
    pub async fn get_guild(&self, guild_id: Snowflake) -> Result<Guild> {
        let url = format!("{}/guilds/{}", self.base_url, guild_id);
        let res = self.check_response(self.client.get(&url).send().await?).await?;
        Ok(res.json().await?)
    }

    /// Fetch all guilds the current user is in.
    pub async fn get_my_guilds(&self) -> Result<Vec<serde_json::Value>> {
        let url = format!("{}/users/@me/guilds", self.base_url);
        let res = self.check_response(self.client.get(&url).send().await?).await?;
        Ok(res.json().await?)
    }

    /// Leave a guild.
    pub async fn leave_guild(&self, guild_id: Snowflake) -> Result<()> {
        let url = format!("{}/users/@me/guilds/{}", self.base_url, guild_id);
        self.check_response(self.client.delete(&url).send().await?).await?;
        Ok(())
    }

    // ── Members ───────────────────────────────────────────────────────────────

    /// Fetch a guild member.
    pub async fn get_member(&self, guild_id: Snowflake, user_id: Snowflake) -> Result<GuildMember> {
        let url = format!("{}/guilds/{}/members/{}", self.base_url, guild_id, user_id);
        let res = self.check_response(self.client.get(&url).send().await?).await?;
        Ok(res.json().await?)
    }

    /// Fetch multiple guild members (up to 1000).
    pub async fn get_members(&self, guild_id: Snowflake, limit: u16, after: Option<Snowflake>) -> Result<Vec<GuildMember>> {
        let mut url = format!("{}/guilds/{}/members?limit={}", self.base_url, guild_id, limit.min(1000));
        if let Some(a) = after { url.push_str(&format!("&after={}", a)); }
        let res = self.check_response(self.client.get(&url).send().await?).await?;
        Ok(res.json().await?)
    }

    /// Kick a member from a guild.
    pub async fn kick_member(&self, guild_id: Snowflake, user_id: Snowflake) -> Result<()> {
        let url = format!("{}/guilds/{}/members/{}", self.base_url, guild_id, user_id);
        self.check_response(self.client.delete(&url).send().await?).await?;
        Ok(())
    }

    /// Ban a member from a guild.
    pub async fn ban_member(&self, guild_id: Snowflake, user_id: Snowflake, delete_message_seconds: u32, reason: Option<&str>) -> Result<()> {
        let url = format!("{}/guilds/{}/bans/{}", self.base_url, guild_id, user_id);
        let mut payload = json!({ "delete_message_seconds": delete_message_seconds });
        if let Some(r) = reason {
            payload["reason"] = json!(r);
        }
        self.check_response(self.client.put(&url).json(&payload).send().await?).await?;
        Ok(())
    }

    /// Unban a user from a guild.
    pub async fn unban_member(&self, guild_id: Snowflake, user_id: Snowflake) -> Result<()> {
        let url = format!("{}/guilds/{}/bans/{}", self.base_url, guild_id, user_id);
        self.check_response(self.client.delete(&url).send().await?).await?;
        Ok(())
    }

    /// Fetch the ban list for a guild.
    pub async fn get_bans(&self, guild_id: Snowflake) -> Result<Vec<Ban>> {
        let url = format!("{}/guilds/{}/bans", self.base_url, guild_id);
        let res = self.check_response(self.client.get(&url).send().await?).await?;
        Ok(res.json().await?)
    }

    /// Add a role to a guild member.
    pub async fn add_member_role(&self, guild_id: Snowflake, user_id: Snowflake, role_id: Snowflake) -> Result<()> {
        let url = format!("{}/guilds/{}/members/{}/roles/{}", self.base_url, guild_id, user_id, role_id);
        self.check_response(self.client.put(&url).send().await?).await?;
        Ok(())
    }

    /// Remove a role from a guild member.
    pub async fn remove_member_role(&self, guild_id: Snowflake, user_id: Snowflake, role_id: Snowflake) -> Result<()> {
        let url = format!("{}/guilds/{}/members/{}/roles/{}", self.base_url, guild_id, user_id, role_id);
        self.check_response(self.client.delete(&url).send().await?).await?;
        Ok(())
    }

    /// Change a member's nickname. Pass `None` to reset it.
    pub async fn set_nickname(&self, guild_id: Snowflake, user_id: Snowflake, nick: Option<&str>) -> Result<()> {
        let url = format!("{}/guilds/{}/members/{}", self.base_url, guild_id, user_id);
        self.check_response(
            self.client.patch(&url).json(&json!({ "nick": nick })).send().await?,
        ).await?;
        Ok(())
    }

    // ── Roles ─────────────────────────────────────────────────────────────────

    /// Fetch all roles in a guild.
    pub async fn get_roles(&self, guild_id: Snowflake) -> Result<Vec<Role>> {
        let url = format!("{}/guilds/{}/roles", self.base_url, guild_id);
        let res = self.check_response(self.client.get(&url).send().await?).await?;
        Ok(res.json().await?)
    }

    /// Create a role in a guild.
    pub async fn create_role(&self, guild_id: Snowflake, payload: serde_json::Value) -> Result<Role> {
        let url = format!("{}/guilds/{}/roles", self.base_url, guild_id);
        let res = self.check_response(self.client.post(&url).json(&payload).send().await?).await?;
        Ok(res.json().await?)
    }

    /// Delete a role from a guild.
    pub async fn delete_role(&self, guild_id: Snowflake, role_id: Snowflake) -> Result<()> {
        let url = format!("{}/guilds/{}/roles/{}", self.base_url, guild_id, role_id);
        self.check_response(self.client.delete(&url).send().await?).await?;
        Ok(())
    }

    // ── Invites ───────────────────────────────────────────────────────────────

    /// Fetch an invite by code.
    pub async fn get_invite(&self, code: &str) -> Result<Invite> {
        let url = format!("{}/invites/{}", self.base_url, code);
        let res = self.check_response(self.client.get(&url).send().await?).await?;
        Ok(res.json().await?)
    }

    /// Delete an invite by code.
    pub async fn delete_invite(&self, code: &str) -> Result<()> {
        let url = format!("{}/invites/{}", self.base_url, code);
        self.check_response(self.client.delete(&url).send().await?).await?;
        Ok(())
    }

    // ── Compat aliases (keep old names working) ───────────────────────────────

    #[inline]
    pub async fn get_current_user(&self) -> Result<User> { self.get_me().await }

    #[inline]
    pub async fn typing(&self, channel_id: Snowflake) -> Result<()> { self.start_typing(channel_id).await }
}
