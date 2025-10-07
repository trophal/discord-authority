use crate::{Error, Result};
use crate::utils::Snowflake;
use crate::models::{Message, Channel, User};
use reqwest::{Client as ReqwestClient, header};
use serde_json::json;
use std::sync::Arc;
use tracing::{debug, info};

pub struct HttpClient {
    client: ReqwestClient,
    token: String,
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
            header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"),
        );

        let client = ReqwestClient::builder()
            .default_headers(headers)
            .cookie_store(true)
            .build()
            .unwrap();

        Self {
            client,
            token,
            base_url: "https://discord.com/api/v9".to_string(),
        }
    }

    pub async fn get_current_user(&self) -> Result<User> {
        let url = format!("{}/users/@me", self.base_url);
        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            return Err(Error::DiscordApi {
                code: status.as_u16() as u64,
                message: text,
            });
        }

        let user = response.json().await?;
        Ok(user)
    }

    pub async fn get_channel(&self, channel_id: Snowflake) -> Result<Channel> {
        let url = format!("{}/channels/{}", self.base_url, channel_id);
        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            return Err(Error::DiscordApi {
                code: status.as_u16() as u64,
                message: text,
            });
        }

        let channel = response.json().await?;
        Ok(channel)
    }

    pub async fn send_message(
        &self,
        channel_id: Snowflake,
        payload: serde_json::Value,
    ) -> Result<Message> {
        let url = format!("{}/channels/{}/messages", self.base_url, channel_id);
        
        debug!("Sending message to channel {}: {:?}", channel_id, payload);
        
        let response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            return Err(Error::DiscordApi {
                code: status.as_u16() as u64,
                message: text,
            });
        }

        let message = response.json().await?;
        Ok(message)
    }

    pub async fn edit_message(
        &self,
        channel_id: Snowflake,
        message_id: Snowflake,
        payload: serde_json::Value,
    ) -> Result<Message> {
        let url = format!("{}/channels/{}/messages/{}", self.base_url, channel_id, message_id);
        
        let response = self.client
            .patch(&url)
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            return Err(Error::DiscordApi {
                code: status.as_u16() as u64,
                message: text,
            });
        }

        let message = response.json().await?;
        Ok(message)
    }

    pub async fn delete_message(
        &self,
        channel_id: Snowflake,
        message_id: Snowflake,
    ) -> Result<()> {
        let url = format!("{}/channels/{}/messages/{}", self.base_url, channel_id, message_id);
        
        let response = self.client
            .delete(&url)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            return Err(Error::DiscordApi {
                code: status.as_u16() as u64,
                message: text,
            });
        }

        Ok(())
    }

    pub async fn get_message(
        &self,
        channel_id: Snowflake,
        message_id: Snowflake,
    ) -> Result<Message> {
        let url = format!("{}/channels/{}/messages/{}", self.base_url, channel_id, message_id);
        
        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            return Err(Error::DiscordApi {
                code: status.as_u16() as u64,
                message: text,
            });
        }

        let message = response.json().await?;
        Ok(message)
    }

    pub async fn add_reaction(
        &self,
        channel_id: Snowflake,
        message_id: Snowflake,
        emoji: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/channels/{}/messages/{}/reactions/{}/@me",
            self.base_url, channel_id, message_id, emoji
        );
        
        let response = self.client
            .put(&url)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            return Err(Error::DiscordApi {
                code: status.as_u16() as u64,
                message: text,
            });
        }

        Ok(())
    }

    pub async fn remove_reaction(
        &self,
        channel_id: Snowflake,
        message_id: Snowflake,
        emoji: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/channels/{}/messages/{}/reactions/{}/@me",
            self.base_url, channel_id, message_id, emoji
        );
        
        let response = self.client
            .delete(&url)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            return Err(Error::DiscordApi {
                code: status.as_u16() as u64,
                message: text,
            });
        }

        Ok(())
    }

    pub async fn vote_poll(
        &self,
        channel_id: Snowflake,
        message_id: Snowflake,
        answer_id: u32,
    ) -> Result<()> {
        let url = format!(
            "{}/channels/{}/polls/{}/answers/{}",
            self.base_url, channel_id, message_id, answer_id
        );
        
        let response = self.client
            .put(&url)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            return Err(Error::DiscordApi {
                code: status.as_u16() as u64,
                message: text,
            });
        }

        Ok(())
    }

    pub async fn typing(&self, channel_id: Snowflake) -> Result<()> {
        let url = format!("{}/channels/{}/typing", self.base_url, channel_id);
        
        let response = self.client
            .post(&url)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            return Err(Error::DiscordApi {
                code: status.as_u16() as u64,
                message: text,
            });
        }

        Ok(())
    }
}

