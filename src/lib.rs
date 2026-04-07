//! # authority
//!
//! A fast, async Discord selfbot library for Rust.
//!
//! ## Warning
//! Using selfbots is against Discord's Terms of Service and can result in account termination.
//! Use this library at your own risk.
//!
//! ## Example
//! ```no_run
//! use authority::{Client, ClientBuilder, EventHandler, User};
//! use std::sync::Arc;
//! use async_trait::async_trait;
//!
//! struct MyHandler;
//!
//! #[async_trait]
//! impl EventHandler for MyHandler {
//!     async fn on_ready(&self, user: User) {
//!         println!("{} is ready!", user.username);
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = ClientBuilder::new("YOUR_TOKEN")
//!         .event_handler(Arc::new(MyHandler))
//!         .build()
//!         .await?;
//!     
//!     client.listen().await?;
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
pub mod gateway;
pub mod http;
pub mod models;
pub mod events;
pub mod utils;

// Re-exports
pub use client::{Client, ClientBuilder};
pub use error::{Error, Result};
pub use models::{
    channel::{Channel, Invite},
    guild::{Guild, GuildMember, Role, Ban},
    message::{Message, MessageBuilder},
    user::User,
    embed::Embed,
    presence::{Activity, Presence, CustomStatus, RichPresence, SpotifyRPC},
    poll::Poll,
};
pub use events::{EventHandler, ReactionEvent};

