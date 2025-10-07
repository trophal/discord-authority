use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Discord API error: {code} - {message}")]
    DiscordApi { code: u64, message: String },

    #[error("Authentication failed: {0}")]
    Authentication(String),

    #[error("Invalid token")]
    InvalidToken,

    #[error("Connection closed: {0}")]
    ConnectionClosed(String),

    #[error("Gateway error: {0}")]
    Gateway(String),

    #[error("Invalid message: {0}")]
    InvalidMessage(String),

    #[error("Rate limited")]
    RateLimit,

    #[error("Unknown error: {0}")]
    Unknown(String),
}

