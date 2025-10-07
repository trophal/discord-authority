use discord_selfbot::{Client, ClientBuilder, EventHandler, Message, User, Poll, MessageBuilder};
use std::sync::Arc;
use async_trait::async_trait;

struct MyEventHandler;

#[async_trait]
impl EventHandler for MyEventHandler {
    async fn ready(&self, user: User) {
        println!("{} is ready!", user.username);
    }

    async fn message_create(&self, message: Message) {
        if message.content == "poll" {
            println!("Poll command received!");
        }
    }

    async fn poll_vote_add(&self, user_id: discord_selfbot::utils::Snowflake, _channel_id: discord_selfbot::utils::Snowflake, _message_id: discord_selfbot::utils::Snowflake, answer_id: u32) {
        println!("User {} voted for answer {}", user_id, answer_id);
    }

    async fn poll_vote_remove(&self, user_id: discord_selfbot::utils::Snowflake, _channel_id: discord_selfbot::utils::Snowflake, _message_id: discord_selfbot::utils::Snowflake, answer_id: u32) {
        println!("User {} removed their vote for answer {}", user_id, answer_id);
    }

    async fn message_update(&self, _old: Option<Message>, new: Message) {
        if new.poll.is_some() {
            println!("Poll was updated!");
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

    // Send poll example (uncomment and set channel_id)
    // let channel_id = "YOUR_CHANNEL_ID".into();
    // let poll = Poll::new("What is your favorite color?")
    //     .add_answer("Red", Some("🍎".to_string()))
    //     .add_answer("Green", Some("🥗".to_string()))
    //     .add_answer("Blue", Some("💙".to_string()))
    //     .add_answer("Yellow", Some("🟡".to_string()))
    //     .duration_hours(8)
    //     .allow_multiselect(true);
    //
    // let payload = MessageBuilder::new()
    //     .poll(poll)
    //     .build();
    //
    // let message = client.send_message_advanced(channel_id, payload).await?;
    // println!("Poll sent! Message ID: {}", message.id);
    //
    // // Vote on poll (multi select)
    // tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    // client.vote_poll(channel_id, message.id, 1).await?;
    // client.vote_poll(channel_id, message.id, 3).await?;
    // println!("Voted on poll!");

    client.listen().await?;

    Ok(())
}

