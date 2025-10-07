use discord_selfbot::{Client, ClientBuilder, Embed, EventHandler, Message, User, MessageBuilder};
use std::sync::Arc;
use async_trait::async_trait;

struct MyEventHandler;

#[async_trait]
impl EventHandler for MyEventHandler {
    async fn ready(&self, user: User) {
        println!("{} is ready!", user.username);
    }

    async fn message_create(&self, message: Message) {
        if message.content == "embed" {
            println!("Embed command received from: {}", message.author.username);
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

    // Send embed example (uncomment and set channel_id)
    // let channel_id = "YOUR_CHANNEL_ID".into();
    // let embed = Embed::new()
    //     .title("Amazing Title")
    //     .description("This is a description")
    //     .color(0xFF0000) // RED
    //     .field("Field 1", "Value 1", true)
    //     .field("Field 2", "Value 2", true)
    //     .field("Field 3", "Value 3", false)
    //     .footer("Footer text")
    //     .image("https://i.ytimg.com/vi/iBP8HambzpY/maxresdefault.jpg")
    //     .thumbnail("https://discord.com/assets/logo.png")
    //     .author_with_url("Author Name", "https://example.com");
    //
    // let payload = MessageBuilder::new()
    //     .content("Check out this embed!")
    //     .embed(embed)
    //     .build();
    //
    // client.send_message_advanced(channel_id, payload).await?;
    // println!("Embed sent!");

    client.listen().await?;

    Ok(())
}

