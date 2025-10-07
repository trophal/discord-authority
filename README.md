# Discord Selfbot - Rust 🦀

A powerful Discord selfbot library written in Rust. Fast, safe, and easy to use.

> ⚠️ **HEADS UP**: Using selfbots violates Discord's ToS and can get your account banned. Use at your own risk!

## What's This?

This is a Rust library that lets you control your Discord account programmatically. Think of it as discord.js but for selfbots and written in Rust. It's blazingly fast thanks to Rust's performance and has a clean API that won't make you cry.

## Features

- ✅ **WebSocket Gateway** - Real-time events, just works
- ✅ **REST API** - Everything you need from Discord's API
- ✅ **Rich Presence** - Flex with custom activities
- ✅ **Custom Status** - Show off what you're doing
- ✅ **Spotify RPC** - Fake or real, your choice
- ✅ **Embeds** - Pretty messages that stand out
- ✅ **Polls** - Create and vote on polls
- ✅ **Async/Await** - Powered by Tokio
- ✅ **Type Safe** - Rust's got your back

## Getting Started

Add this to your `Cargo.toml`:

```toml
[dependencies]
discord-selfbot = { path = "." }
```

Or if you're working locally:

```toml
[dependencies]
discord-selfbot = { git = "https://github.com/ege0x77czz/rust-discord-selfbot" }
```

## Quick Examples

### Basic Setup

```rust
use discord_selfbot::{Client, ClientBuilder, EventHandler, Message, User};
use std::sync::Arc;
use async_trait::async_trait;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, user: User) {
        println!("{} is ready!", user.username);
    }

    async fn message_create(&self, message: Message) {
        if message.content == "ping" {
            println!("Got ping from {}", message.author.username);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::new("YOUR_TOKEN_HERE")
        .event_handler(Arc::new(Handler))
        .build()
        .await?;

    client.listen().await?;
    Ok(())
}
```

### Send a Message

```rust
let channel_id = "123456789".into();
client.send_message(channel_id, "yo what's up").await?;
```

### Send an Embed

```rust
use discord_selfbot::{Embed, MessageBuilder};

let embed = Embed::new()
    .title("Check This Out")
    .description("Pretty cool right?")
    .color(0xFF0000)
    .field("Field 1", "Value 1", true)
    .footer("Made with Rust")
    .image("https://example.com/image.png");

let payload = MessageBuilder::new()
    .content("Look at this embed:")
    .embed(embed)
    .build();

client.send_message_advanced(channel_id, payload).await?;
```

### Rich Presence (Flex Mode)

```rust
use discord_selfbot::RichPresence;

let presence = RichPresence::new("Minecraft")
    .state("Building stuff")
    .details("Survival Mode")
    .large_image("minecraft_icon")
    .large_text("Version 1.20")
    .party(2, 10)
    .start_timestamp(discord_selfbot::utils::now())
    .add_button("Join Me", "https://example.com/join")
    .to_activity();

client.set_activity(presence).await?;
```

### Custom Status

```rust
use discord_selfbot::CustomStatus;

let status = CustomStatus::new()
    .emoji("🔥")
    .state("grinding in Rust")
    .to_activity();

client.set_activity(status).await?;
```

### Create a Poll

```rust
use discord_selfbot::{Poll, MessageBuilder};

let poll = Poll::new("What's your favorite language?")
    .add_answer("Rust", Some("🦀".to_string()))
    .add_answer("Python", Some("🐍".to_string()))
    .add_answer("JavaScript", Some("💩".to_string()))
    .duration_hours(24)
    .allow_multiselect(false);

let payload = MessageBuilder::new().poll(poll).build();
let message = client.send_message_advanced(channel_id, payload).await?;

// Vote for Rust obviously
client.vote_poll(channel_id, message.id, 1).await?;
```

## Examples

Check out the `examples/` folder for more:

- `basic.rs` - Simple bot to get started
- `embed.rs` - Sending fancy embeds
- `rich_presence.rs` - Show off with Rich Presence
- `poll.rs` - Polls and voting

Run them like this:

```bash
cargo run --example basic
cargo run --example rich_presence
```

**Note:** Don't forget to replace `"token"` with your actual Discord token in the code!

## Getting Your Token

Open Discord, press `Ctrl + Shift + I` to open DevTools, go to Console, and paste this:

```javascript
window.webpackChunkdiscord_app.push([
    [Symbol()],
    {},
    req => {
        if (!req.c) return;
        for (let m of Object.values(req.c)) {
            try {
                if (!m.exports || m.exports === window) continue;
                if (m.exports?.getToken) return copy(m.exports.getToken());
                for (let ex in m.exports) {
                    if (m.exports?.[ex]?.getToken && m.exports[ex][Symbol.toStringTag] !== 'IntlMessagesProxy') 
                        return copy(m.exports[ex].getToken());
                }
            } catch {}
        }
    },
]);

window.webpackChunkdiscord_app.pop();
```

Your token will be copied to clipboard. **Keep it secret!**

## All Events

Here's everything you can listen to:

```rust
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, user: User) {}
    async fn message_create(&self, message: Message) {}
    async fn message_update(&self, old: Option<Message>, new: Message) {}
    async fn message_delete(&self, channel_id: Snowflake, message_id: Snowflake) {}
    async fn guild_create(&self, guild: Guild) {}
    async fn guild_delete(&self, guild_id: Snowflake) {}
    async fn channel_create(&self, channel: Channel) {}
    async fn channel_delete(&self, channel: Channel) {}
    async fn presence_update(&self, presence: Presence) {}
    async fn typing_start(&self, channel_id: Snowflake, user_id: Snowflake) {}
    async fn poll_vote_add(&self, user_id: Snowflake, channel_id: Snowflake, message_id: Snowflake, answer_id: u32) {}
    async fn poll_vote_remove(&self, user_id: Snowflake, channel_id: Snowflake, message_id: Snowflake, answer_id: u32) {}
    async fn raw(&self, event: serde_json::Value) {}
}
```

## What You Can Do

### Client Methods

- `send_message(channel_id, content)` - Send a quick message
- `send_message_advanced(channel_id, payload)` - Send message with embeds, polls, etc.
- `edit_message(channel_id, message_id, content)` - Edit your messages
- `delete_message(channel_id, message_id)` - Delete messages
- `get_message(channel_id, message_id)` - Fetch a message
- `add_reaction(channel_id, message_id, emoji)` - React to messages
- `remove_reaction(channel_id, message_id, emoji)` - Remove reactions
- `vote_poll(channel_id, message_id, answer_id)` - Vote on polls
- `typing(channel_id)` - Show typing indicator
- `set_presence(activities, status)` - Set your presence
- `set_activity(activity)` - Set a single activity
- `set_status(status)` - Just change your status

### Message Helpers

```rust
message.reply(http, "sup").await?;
message.react(http, "👍").await?;
message.delete(http).await?;
message.edit(http, "fixed typo").await?;
```

## Requirements

- Rust 1.70+ (probably works on older versions too but haven't tested)
- Tokio runtime

## Contributing

Found a bug? Want to add something cool? PRs are welcome! Just open an issue first if it's something big.

## Legal Stuff

This is for educational purposes. Using selfbots is against Discord's Terms of Service. Your account might get banned. We're not responsible if that happens. You've been warned! ⚠️

Also, this project is licensed under MIT. Do whatever you want with it.

## Need Help?

Open an issue on GitHub if you're stuck or something's broken.

---

**Remember:** This breaks Discord's ToS. Use responsibly (or don't get caught 😉)
