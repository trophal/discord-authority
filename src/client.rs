use crate::{Error, Result};
use crate::events::{EventHandler, DefaultEventHandler};
use crate::gateway::Gateway;
use crate::http::HttpClient;
use crate::models::{User, Message, Activity};
use crate::utils::Snowflake;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

pub struct Client {
    gateway: Arc<Gateway>,
    http: Arc<HttpClient>,
    user: Arc<RwLock<Option<User>>>,
}

impl Client {
    /// Get the current user
    pub async fn user(&self) -> Option<User> {
        self.user.read().await.clone()
    }

    /// Send a message to a channel
    pub async fn send_message(&self, channel_id: Snowflake, content: impl Into<String>) -> Result<Message> {
        let payload = serde_json::json!({
            "content": content.into()
        });
        self.http.send_message(channel_id, payload).await
    }

    /// Send a message with custom payload
    pub async fn send_message_advanced(&self, channel_id: Snowflake, payload: serde_json::Value) -> Result<Message> {
        self.http.send_message(channel_id, payload).await
    }

    /// Edit a message
    pub async fn edit_message(&self, channel_id: Snowflake, message_id: Snowflake, content: impl Into<String>) -> Result<Message> {
        let payload = serde_json::json!({
            "content": content.into()
        });
        self.http.edit_message(channel_id, message_id, payload).await
    }

    /// Delete a message
    pub async fn delete_message(&self, channel_id: Snowflake, message_id: Snowflake) -> Result<()> {
        self.http.delete_message(channel_id, message_id).await
    }

    /// Get a message
    pub async fn get_message(&self, channel_id: Snowflake, message_id: Snowflake) -> Result<Message> {
        self.http.get_message(channel_id, message_id).await
    }

    /// Add a reaction to a message
    pub async fn add_reaction(&self, channel_id: Snowflake, message_id: Snowflake, emoji: &str) -> Result<()> {
        self.http.add_reaction(channel_id, message_id, emoji).await
    }

    /// Remove a reaction from a message
    pub async fn remove_reaction(&self, channel_id: Snowflake, message_id: Snowflake, emoji: &str) -> Result<()> {
        self.http.remove_reaction(channel_id, message_id, emoji).await
    }

    /// Vote on a poll
    pub async fn vote_poll(&self, channel_id: Snowflake, message_id: Snowflake, answer_id: u32) -> Result<()> {
        self.http.vote_poll(channel_id, message_id, answer_id).await
    }

    /// Start typing indicator
    pub async fn typing(&self, channel_id: Snowflake) -> Result<()> {
        self.http.typing(channel_id).await
    }

    /// Update presence (status and activities)
    pub async fn set_presence(&self, activities: Vec<Activity>, status: &str) -> Result<()> {
        self.gateway.update_presence(activities, status).await
    }

    /// Set a single activity
    pub async fn set_activity(&self, activity: Activity) -> Result<()> {
        self.gateway.update_presence(vec![activity], "online").await
    }

    /// Set status only
    pub async fn set_status(&self, status: &str) -> Result<()> {
        self.gateway.update_presence(vec![], status).await
    }

    /// Start listening for events
    pub async fn listen(&self) -> Result<()> {
        self.gateway.listen().await
    }

    /// Get HTTP client reference
    pub fn http(&self) -> &HttpClient {
        &self.http
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
        // Initialize tracing
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", "info");
        }
        
        tracing_subscriber::fmt::init();

        let token = self.token;
        let event_handler = self.event_handler.unwrap_or_else(|| Arc::new(DefaultEventHandler));

        // Create HTTP client
        let http = Arc::new(HttpClient::new(token.clone()));

        // Verify token by fetching current user
        info!("Verifying token...");
        let user = http.get_current_user().await.map_err(|_| Error::InvalidToken)?;
        info!("Authenticated as: {}", user.tag());

        // Create gateway
        let gateway = Arc::new(Gateway::new(token, event_handler));

        // Connect to gateway
        info!("Connecting to Discord Gateway...");
        gateway.connect().await?;

        let client = Client {
            gateway,
            http,
            user: Arc::new(RwLock::new(Some(user))),
        };

        Ok(client)
    }
}

/// Helper methods for Message
impl Message {
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

    pub async fn react(&self, http: &HttpClient, emoji: &str) -> Result<()> {
        http.add_reaction(self.channel_id, self.id, emoji).await
    }

    pub async fn delete(&self, http: &HttpClient) -> Result<()> {
        http.delete_message(self.channel_id, self.id).await
    }

    pub async fn edit(&self, http: &HttpClient, content: impl Into<String>) -> Result<Message> {
        let payload = serde_json::json!({
            "content": content.into()
        });
        http.edit_message(self.channel_id, self.id, payload).await
    }
}

