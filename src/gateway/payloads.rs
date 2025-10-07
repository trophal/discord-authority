use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayPayload<T> {
    pub op: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hello {
    pub heartbeat_interval: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ready {
    pub v: u8,
    pub user: serde_json::Value,
    pub guilds: Vec<serde_json::Value>,
    pub session_id: String,
    pub resume_gateway_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identify {
    pub token: String,
    pub properties: ConnectionProperties,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compress: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_threshold: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard: Option<[u16; 2]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionProperties {
    #[serde(rename = "$os")]
    pub os: String,
    #[serde(rename = "$browser")]
    pub browser: String,
    #[serde(rename = "$device")]
    pub device: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resume {
    pub token: String,
    pub session_id: String,
    pub seq: u64,
}

