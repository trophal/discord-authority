pub mod opcodes;
pub mod payloads;

use crate::{Error, Result};
use crate::events::EventHandler;
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::{
    connect_async, tungstenite::Message as WsMessage, MaybeTlsStream, WebSocketStream,
};
use tracing::{debug, error, info, warn};

pub use opcodes::OpCode;
pub use payloads::*;

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct Gateway {
    ws: Arc<Mutex<Option<WsStream>>>,
    token: String,
    session_id: Arc<Mutex<Option<String>>>,
    sequence: Arc<Mutex<Option<u64>>>,
    heartbeat_interval: Arc<Mutex<Option<u64>>>,
    event_handler: Arc<dyn EventHandler>,
}

impl Gateway {
    pub fn new(token: String, event_handler: Arc<dyn EventHandler>) -> Self {
        Self {
            ws: Arc::new(Mutex::new(None)),
            token,
            session_id: Arc::new(Mutex::new(None)),
            sequence: Arc::new(Mutex::new(None)),
            heartbeat_interval: Arc::new(Mutex::new(None)),
            event_handler,
        }
    }

    pub async fn connect(&self) -> Result<()> {
        let gateway_url = "wss://gateway.discord.gg/?v=9&encoding=json";
        
        info!("Connecting to Discord Gateway: {}", gateway_url);
        
        let (ws_stream, _) = connect_async(gateway_url)
            .await
            .map_err(|e| Error::Gateway(format!("Failed to connect: {}", e)))?;

        info!("Connected to Discord Gateway");

        let mut ws_lock = self.ws.lock().await;
        *ws_lock = Some(ws_stream);
        drop(ws_lock);

        Ok(())
    }

    pub async fn identify(&self) -> Result<()> {
        let identify_payload = json!({
            "op": OpCode::Identify as u8,
            "d": {
                "token": self.token,
                "properties": {
                    "$os": std::env::consts::OS,
                    "$browser": "discord-selfbot-rust",
                    "$device": "discord-selfbot-rust"
                },
                "compress": false,
                "large_threshold": 250,
                "presence": {
                    "status": "online",
                    "since": 0,
                    "activities": [],
                    "afk": false
                }
            }
        });

        self.send(identify_payload).await?;
        info!("Sent IDENTIFY payload");

        Ok(())
    }

    pub async fn send(&self, payload: serde_json::Value) -> Result<()> {
        let mut ws_lock = self.ws.lock().await;
        
        if let Some(ref mut ws) = *ws_lock {
            let message = serde_json::to_string(&payload)?;
            ws.send(WsMessage::Text(message)).await?;
            Ok(())
        } else {
            Err(Error::Gateway("WebSocket not connected".to_string()))
        }
    }

    pub async fn start_heartbeat(&self) {
        let interval = {
            let interval_lock = self.heartbeat_interval.lock().await;
            *interval_lock
        };

        if let Some(interval_ms) = interval {
            let ws = self.ws.clone();
            let sequence = self.sequence.clone();
            
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(
                    tokio::time::Duration::from_millis(interval_ms)
                );
                
                loop {
                    interval.tick().await;
                    
                    let seq = {
                        let seq_lock = sequence.lock().await;
                        *seq_lock
                    };

                    let heartbeat = json!({
                        "op": OpCode::Heartbeat as u8,
                        "d": seq
                    });

                    let mut ws_lock = ws.lock().await;
                    if let Some(ref mut stream) = *ws_lock {
                        if let Ok(msg) = serde_json::to_string(&heartbeat) {
                            if stream.send(WsMessage::Text(msg)).await.is_err() {
                                error!("Failed to send heartbeat");
                                break;
                            }
                            debug!("Sent heartbeat");
                        }
                    }
                }
            });
        }
    }

    pub async fn listen(&self) -> Result<()> {
        loop {
            let message = {
                let mut ws_lock = self.ws.lock().await;
                if let Some(ref mut ws) = *ws_lock {
                    match ws.next().await {
                        Some(Ok(msg)) => msg,
                        Some(Err(e)) => {
                            error!("WebSocket error: {}", e);
                            return Err(Error::WebSocket(e));
                        }
                        None => {
                            warn!("WebSocket stream ended");
                            return Err(Error::ConnectionClosed("Stream ended".to_string()));
                        }
                    }
                } else {
                    return Err(Error::Gateway("WebSocket not connected".to_string()));
                }
            };

            match message {
                WsMessage::Text(text) => {
                    if let Err(e) = self.handle_message(&text).await {
                        error!("Error handling message: {}", e);
                    }
                }
                WsMessage::Close(_) => {
                    warn!("Received close frame");
                    return Err(Error::ConnectionClosed("Close frame received".to_string()));
                }
                _ => {}
            }
        }
    }

    async fn handle_message(&self, text: &str) -> Result<()> {
        let payload: serde_json::Value = serde_json::from_str(text)?;
        
        let op = payload["op"].as_u64().unwrap_or(999);
        let seq = payload["s"].as_u64();
        
        if let Some(s) = seq {
            let mut seq_lock = self.sequence.lock().await;
            *seq_lock = Some(s);
        }

        match op {
            10 => {
                // Hello
                if let Some(interval) = payload["d"]["heartbeat_interval"].as_u64() {
                    let mut interval_lock = self.heartbeat_interval.lock().await;
                    *interval_lock = Some(interval);
                    info!("Received HELLO, heartbeat interval: {}ms", interval);
                    drop(interval_lock);
                    self.start_heartbeat().await;
                    self.identify().await?;
                }
            }
            11 => {
                // Heartbeat ACK
                debug!("Received heartbeat ACK");
            }
            0 => {
                // Dispatch
                if let Some(event_type) = payload["t"].as_str() {
                    self.handle_event(event_type, &payload["d"]).await?;
                }
            }
            _ => {
                debug!("Unknown opcode: {}", op);
            }
        }

        Ok(())
    }

    async fn handle_event(&self, event_type: &str, data: &serde_json::Value) -> Result<()> {
        debug!("Received event: {}", event_type);

        self.event_handler.on_raw_event(data.clone()).await;

        match event_type {
            "READY" => {
                if let Ok(user) = serde_json::from_value(data["user"].clone()) {
                    if let Some(session_id) = data["session_id"].as_str() {
                        let mut session_lock = self.session_id.lock().await;
                        *session_lock = Some(session_id.to_string());
                    }
                    info!("Client is ready!");
                    self.event_handler.on_ready(user).await;
                }
            }
            "RESUMED" => {
                info!("Session resumed");
                self.event_handler.on_resumed().await;
            }

            // ── Messages ──────────────────────────────────────────────────
            "MESSAGE_CREATE" => {
                if let Ok(message) = serde_json::from_value(data.clone()) {
                    self.event_handler.on_message(message).await;
                }
            }
            "MESSAGE_UPDATE" => {
                if let Ok(new_message) = serde_json::from_value(data.clone()) {
                    self.event_handler.on_message_edit(None, new_message).await;
                }
            }
            "MESSAGE_DELETE" => {
                if let (Some(channel_id), Some(message_id)) = (
                    data["channel_id"].as_str(),
                    data["id"].as_str(),
                ) {
                    self.event_handler.on_message_delete(channel_id.into(), message_id.into()).await;
                }
            }
            "MESSAGE_DELETE_BULK" => {
                if let Some(channel_id) = data["channel_id"].as_str() {
                    let ids = data["ids"]
                        .as_array()
                        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.into())).collect())
                        .unwrap_or_default();
                    self.event_handler.on_message_bulk_delete(channel_id.into(), ids).await;
                }
            }

            // ── Reactions ─────────────────────────────────────────────────
            "MESSAGE_REACTION_ADD" => {
                if let (Some(user_id), Some(channel_id), Some(message_id)) = (
                    data["user_id"].as_str(),
                    data["channel_id"].as_str(),
                    data["message_id"].as_str(),
                ) {
                    let emoji = data["emoji"]["name"].as_str().unwrap_or("").to_string();
                    let guild_id = data["guild_id"].as_str().map(|s| s.into());
                    self.event_handler.on_reaction_add(crate::events::ReactionEvent {
                        user_id: user_id.into(),
                        channel_id: channel_id.into(),
                        message_id: message_id.into(),
                        guild_id,
                        emoji,
                    }).await;
                }
            }
            "MESSAGE_REACTION_REMOVE" => {
                if let (Some(user_id), Some(channel_id), Some(message_id)) = (
                    data["user_id"].as_str(),
                    data["channel_id"].as_str(),
                    data["message_id"].as_str(),
                ) {
                    let emoji = data["emoji"]["name"].as_str().unwrap_or("").to_string();
                    let guild_id = data["guild_id"].as_str().map(|s| s.into());
                    self.event_handler.on_reaction_remove(crate::events::ReactionEvent {
                        user_id: user_id.into(),
                        channel_id: channel_id.into(),
                        message_id: message_id.into(),
                        guild_id,
                        emoji,
                    }).await;
                }
            }
            "MESSAGE_REACTION_REMOVE_ALL" => {
                if let (Some(channel_id), Some(message_id)) = (
                    data["channel_id"].as_str(),
                    data["message_id"].as_str(),
                ) {
                    self.event_handler.on_reaction_clear(channel_id.into(), message_id.into()).await;
                }
            }
            "MESSAGE_REACTION_REMOVE_EMOJI" => {
                if let (Some(channel_id), Some(message_id)) = (
                    data["channel_id"].as_str(),
                    data["message_id"].as_str(),
                ) {
                    let emoji = data["emoji"]["name"].as_str().unwrap_or("").to_string();
                    self.event_handler.on_reaction_clear_emoji(channel_id.into(), message_id.into(), emoji).await;
                }
            }

            // ── Guilds ────────────────────────────────────────────────────
            "GUILD_CREATE" => {
                if let Ok(guild) = serde_json::from_value(data.clone()) {
                    self.event_handler.on_guild_join(guild).await;
                }
            }
            "GUILD_UPDATE" => {
                if let Ok(guild) = serde_json::from_value(data.clone()) {
                    self.event_handler.on_guild_update(None, guild).await;
                }
            }
            "GUILD_DELETE" => {
                if let Some(guild_id) = data["id"].as_str() {
                    if data["unavailable"].as_bool().unwrap_or(false) {
                        self.event_handler.on_guild_unavailable(guild_id.into()).await;
                    } else {
                        self.event_handler.on_guild_leave(guild_id.into()).await;
                    }
                }
            }

            // ── Members ───────────────────────────────────────────────────
            "GUILD_MEMBER_ADD" => {
                if let (Some(guild_id), Ok(member)) = (
                    data["guild_id"].as_str(),
                    serde_json::from_value(data.clone()),
                ) {
                    self.event_handler.on_member_join(guild_id.into(), member).await;
                }
            }
            "GUILD_MEMBER_UPDATE" => {
                if let (Some(guild_id), Ok(member)) = (
                    data["guild_id"].as_str(),
                    serde_json::from_value(data.clone()),
                ) {
                    self.event_handler.on_member_update(guild_id.into(), member).await;
                }
            }
            "GUILD_MEMBER_REMOVE" => {
                if let (Some(guild_id), Ok(user)) = (
                    data["guild_id"].as_str(),
                    serde_json::from_value(data["user"].clone()),
                ) {
                    self.event_handler.on_member_leave(guild_id.into(), user).await;
                }
            }

            // ── Roles ─────────────────────────────────────────────────────
            "GUILD_ROLE_CREATE" => {
                if let (Some(guild_id), Ok(role)) = (
                    data["guild_id"].as_str(),
                    serde_json::from_value(data["role"].clone()),
                ) {
                    self.event_handler.on_role_create(guild_id.into(), role).await;
                }
            }
            "GUILD_ROLE_UPDATE" => {
                if let (Some(guild_id), Ok(role)) = (
                    data["guild_id"].as_str(),
                    serde_json::from_value(data["role"].clone()),
                ) {
                    self.event_handler.on_role_update(guild_id.into(), role).await;
                }
            }
            "GUILD_ROLE_DELETE" => {
                if let (Some(guild_id), Some(role_id)) = (
                    data["guild_id"].as_str(),
                    data["role_id"].as_str(),
                ) {
                    self.event_handler.on_role_delete(guild_id.into(), role_id.into()).await;
                }
            }

            // ── Channels ──────────────────────────────────────────────────
            "CHANNEL_CREATE" => {
                if let Ok(channel) = serde_json::from_value(data.clone()) {
                    self.event_handler.on_channel_create(channel).await;
                }
            }
            "CHANNEL_UPDATE" => {
                if let Ok(channel) = serde_json::from_value(data.clone()) {
                    self.event_handler.on_channel_update(None, channel).await;
                }
            }
            "CHANNEL_DELETE" => {
                if let Ok(channel) = serde_json::from_value(data.clone()) {
                    self.event_handler.on_channel_delete(channel).await;
                }
            }
            "CHANNEL_PINS_UPDATE" => {
                if let Some(channel_id) = data["channel_id"].as_str() {
                    let last_pin = data["last_pin_timestamp"].as_str().map(|s| s.to_string());
                    self.event_handler.on_channel_pins_update(channel_id.into(), last_pin).await;
                }
            }

            // ── Threads ───────────────────────────────────────────────────
            "THREAD_CREATE" => {
                if let Ok(channel) = serde_json::from_value(data.clone()) {
                    self.event_handler.on_thread_create(channel).await;
                }
            }
            "THREAD_UPDATE" => {
                if let Ok(channel) = serde_json::from_value(data.clone()) {
                    self.event_handler.on_thread_update(channel).await;
                }
            }
            "THREAD_DELETE" => {
                if let Some(channel_id) = data["id"].as_str() {
                    let guild_id = data["guild_id"].as_str().map(|s| s.into());
                    self.event_handler.on_thread_delete(channel_id.into(), guild_id).await;
                }
            }

            // ── Invites ───────────────────────────────────────────────────
            "INVITE_CREATE" => {
                if let Ok(invite) = serde_json::from_value(data.clone()) {
                    self.event_handler.on_invite_create(invite).await;
                }
            }
            "INVITE_DELETE" => {
                if let (Some(channel_id), Some(code)) = (
                    data["channel_id"].as_str(),
                    data["code"].as_str(),
                ) {
                    self.event_handler.on_invite_delete(channel_id.into(), code.to_string()).await;
                }
            }

            // ── Bans ──────────────────────────────────────────────────────
            "GUILD_BAN_ADD" => {
                if let (Some(guild_id), Ok(user)) = (
                    data["guild_id"].as_str(),
                    serde_json::from_value(data["user"].clone()),
                ) {
                    self.event_handler.on_ban_add(guild_id.into(), user).await;
                }
            }
            "GUILD_BAN_REMOVE" => {
                if let (Some(guild_id), Ok(user)) = (
                    data["guild_id"].as_str(),
                    serde_json::from_value(data["user"].clone()),
                ) {
                    self.event_handler.on_ban_remove(guild_id.into(), user).await;
                }
            }

            // ── Presence & Typing ─────────────────────────────────────────
            "PRESENCE_UPDATE" => {
                if let Ok(presence) = serde_json::from_value(data.clone()) {
                    self.event_handler.on_presence_update(presence).await;
                }
            }
            "TYPING_START" => {
                if let (Some(channel_id), Some(user_id)) = (
                    data["channel_id"].as_str(),
                    data["user_id"].as_str(),
                ) {
                    self.event_handler.on_typing_start(channel_id.into(), user_id.into()).await;
                }
            }

            // ── Polls ─────────────────────────────────────────────────────
            "MESSAGE_POLL_VOTE_ADD" => {
                if let (Some(user_id), Some(channel_id), Some(message_id), Some(answer_id)) = (
                    data["user_id"].as_str(),
                    data["channel_id"].as_str(),
                    data["message_id"].as_str(),
                    data["answer_id"].as_u64(),
                ) {
                    self.event_handler.on_poll_vote_add(
                        user_id.into(), channel_id.into(), message_id.into(), answer_id as u32,
                    ).await;
                }
            }
            "MESSAGE_POLL_VOTE_REMOVE" => {
                if let (Some(user_id), Some(channel_id), Some(message_id), Some(answer_id)) = (
                    data["user_id"].as_str(),
                    data["channel_id"].as_str(),
                    data["message_id"].as_str(),
                    data["answer_id"].as_u64(),
                ) {
                    self.event_handler.on_poll_vote_remove(
                        user_id.into(), channel_id.into(), message_id.into(), answer_id as u32,
                    ).await;
                }
            }

            // ── Users ─────────────────────────────────────────────────────
            "USER_UPDATE" => {
                if let Ok(user) = serde_json::from_value(data.clone()) {
                    self.event_handler.on_user_update(user).await;
                }
            }

            _ => {
                debug!("Unhandled event: {}", event_type);
            }
        }

        Ok(())
    }

    pub async fn update_presence(&self, activities: Vec<crate::models::Activity>, status: &str) -> Result<()> {
        let presence_payload = json!({
            "op": OpCode::PresenceUpdate as u8,
            "d": {
                "since": 0,
                "activities": activities,
                "status": status,
                "afk": false
            }
        });

        self.send(presence_payload).await?;
        Ok(())
    }
}

