use discord_selfbot::{
    Client, ClientBuilder, EventHandler, User, 
    RichPresence, CustomStatus, SpotifyRPC,
};
use std::sync::Arc;
use async_trait::async_trait;

struct MyEventHandler {
    client_ref: Arc<tokio::sync::RwLock<Option<Arc<Client>>>>,
}

#[async_trait]
impl EventHandler for MyEventHandler {
    async fn ready(&self, user: User) {
        println!("{} is ready!", user.username);
        
        if let Some(client) = self.client_ref.read().await.as_ref() {
            // Rich Presence
            let rich_presence = RichPresence::new("Minecraft")
                .application_id("1234567890")
                .state("Exploring caves")
                .details("Playing survival mode")
                .large_image("minecraft_icon")
                .large_text("Version 1.20")
                .small_image("pickaxe")
                .small_text("Mining")
                .party(2, 10)
                .start_timestamp(discord_selfbot::utils::now())
                .add_button("Join Server", "https://example.com/join")
                .to_activity();
            
            // Custom Status
            let custom = CustomStatus::new()
                .emoji("🎮")
                .state("Gaming with friends")
                .to_activity();
            
            // Spotify
            let spotify = SpotifyRPC::new()
                .details("Never Gonna Give You Up")
                .state("Rick Astley")
                .large_image("spotify:ab67616d00001e02768629f8bc5b39b68797d1bb")
                .large_text("Whenever You Need Somebody")
                .start_timestamp(discord_selfbot::utils::now())
                .end_timestamp(discord_selfbot::utils::now() + 1_000 * (3 * 60 + 32))
                .song_id("4PTG3Z6ehGkBFwjybzWkR8")
                .to_activity();

            if let Err(e) = client.set_presence(vec![rich_presence, custom, spotify], "online").await {
                eprintln!("Failed to set presence: {}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_ref = Arc::new(tokio::sync::RwLock::new(None));
    
    let handler = Arc::new(MyEventHandler {
        client_ref: client_ref.clone(),
    });
    
    let client = ClientBuilder::new("token")
        .event_handler(handler)
        .build()
        .await?;

    *client_ref.write().await = Some(Arc::new(client));

    if let Some(client) = client_ref.read().await.as_ref() {
        client.listen().await?;
    }

    Ok(())
}

