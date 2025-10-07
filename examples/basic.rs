use discord_selfbot::{Client, ClientBuilder, EventHandler, Message, User};
use std::sync::Arc;
use async_trait::async_trait;

struct MyEventHandler;

#[async_trait]
impl EventHandler for MyEventHandler {
    async fn ready(&self, user: User) {
        println!("{} is ready!", user.username);
    }

    async fn message_create(&self, message: Message) {
        if message.content == "ping" {
            println!("Received ping from: {}", message.author.username);
            // Note: To reply, you need access to the HTTP client
            // message.reply(&http, "pong").await?;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handler = Arc::new(MyEventHandler);
    let client = ClientBuilder::new("token")
        .event_handler(handler)
        .build()
        .await?;

    client.listen().await?;

    Ok(())
}

