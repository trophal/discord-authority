# discord-authority 🦀

A fast, async Discord selfbot library written in Rust.

> ⚠️ **Heads up**: Using selfbots violates Discord's ToS and can get your account banned. Use at your own risk.

## Features

- WebSocket gateway with real-time events
- Full REST API coverage (messages, reactions, guilds, members, roles, bans, invites, polls, DMs)
- Rich Presence, Spotify RPC, and custom status
- Embeds and polls with builder APIs
- Async/await powered by Tokio
- Clean `on_*` event handler naming

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
discord-authority = { git = "https://github.com/trophal/discord-authority" }
```

Or if working locally:

```toml
[dependencies]
discord-authority = { path = "." }
```

## Quick Start

```rust
use discord_authority::{ClientBuilder, EventHandler, Message, User};
use std::sync::Arc;
use async_trait::async_trait;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn on_ready(&self, user: User) {
        println!("{} is ready!", user.username);
    }

    async fn on_message(&self, message: Message) {
        if message.content == "ping" {
            println!("pong from {}", message.author.username);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::new("YOUR_TOKEN")
        .event_handler(Arc::new(Handler))
        .build()
        .await?;

    client.listen().await?;
    Ok(())
}
```

## Examples

### Send a message

```rust
client.send_message(channel_id, "yo what's up").await?;
```

### Send a DM

```rust
client.send_dm(user_id, "hey").await?;
```

### Send an embed

```rust
use discord_authority::Embed;

let embed = Embed::new()
    .title("Check This Out")
    .description("Pretty cool right?")
    .color(0xFF0000)
    .field("Field 1", "Value 1", true)
    .footer("Made with authority");

client.send_embed(channel_id, embed).await?;
```

### Message history

```rust
let messages = client.get_messages(channel_id, 50, None, None).await?;
```

### Reactions

```rust
client.add_reaction(channel_id, message_id, "👍").await?;
client.remove_reaction(channel_id, message_id, "👍").await?;
client.clear_reactions(channel_id, message_id).await?;
```

### Pin / unpin

```rust
client.pin_message(channel_id, message_id).await?;
client.unpin_message(channel_id, message_id).await?;
```

### Bulk delete

```rust
client.bulk_delete_messages(channel_id, vec![id1, id2, id3]).await?;
```

### Guild management

```rust
// Members
let member = client.get_member(guild_id, user_id).await?;
client.kick_member(guild_id, user_id).await?;
client.ban_member(guild_id, user_id, 0, Some("spamming")).await?;
client.unban_member(guild_id, user_id).await?;
let bans = client.get_bans(guild_id).await?;

// Roles
client.add_role(guild_id, user_id, role_id).await?;
client.remove_role(guild_id, user_id, role_id).await?;
let roles = client.get_roles(guild_id).await?;

// Nickname
client.set_nickname(guild_id, user_id, Some("cool nick")).await?;

// Leave
client.leave_guild(guild_id).await?;
```

### Invites

```rust
let invite = client.create_invite(channel_id, 86400, 0, false).await?;
println!("discord.gg/{}", invite.code);
```

### Polls

```rust
use discord_authority::{Poll, MessageBuilder};

let poll = Poll::new("Favorite language?")
    .add_answer("Rust", Some("🦀".to_string()))
    .add_answer("Python", Some("🐍".to_string()))
    .duration_hours(24)
    .allow_multiselect(false);

let msg = client.send(channel_id, MessageBuilder::new().poll(poll)).await?;

client.vote_poll(channel_id, msg.id, 1).await?;
client.unvote_poll(channel_id, msg.id, 1).await?;
```

### Rich Presence

```rust
use discord_authority::RichPresence;

let presence = RichPresence::new("Minecraft")
    .details("Survival Mode")
    .state("Building stuff")
    .large_image("minecraft_icon")
    .large_text("1.20")
    .party(2, 10)
    .start_timestamp(discord_authority::utils::now())
    .add_button("Join", "https://example.com/join")
    .to_activity();

client.set_activity(presence).await?;
```

### Custom Status

```rust
use discord_authority::CustomStatus;

let status = CustomStatus::new()
    .emoji("🔥")
    .state("grinding in Rust")
    .to_activity();

client.set_activity(status).await?;
client.clear_activity().await?;
```

### Message helpers

```rust
message.reply(&client.http, "sup").await?;
message.reply_embed(&client.http, embed).await?;
message.react(&client.http, "👍").await?;
message.edit(&client.http, "fixed typo").await?;
message.pin(&client.http).await?;
message.delete(&client.http).await?;
```

## All Events

```rust
#[async_trait]
impl EventHandler for Handler {
    // Connection
    async fn on_ready(&self, user: User) {}
    async fn on_resumed(&self) {}

    // Messages
    async fn on_message(&self, message: Message) {}
    async fn on_message_edit(&self, old: Option<Message>, new: Message) {}
    async fn on_message_delete(&self, channel_id: Snowflake, message_id: Snowflake) {}
    async fn on_message_bulk_delete(&self, channel_id: Snowflake, message_ids: Vec<Snowflake>) {}

    // Reactions
    async fn on_reaction_add(&self, reaction: ReactionEvent) {}
    async fn on_reaction_remove(&self, reaction: ReactionEvent) {}
    async fn on_reaction_clear(&self, channel_id: Snowflake, message_id: Snowflake) {}
    async fn on_reaction_clear_emoji(&self, channel_id: Snowflake, message_id: Snowflake, emoji: String) {}

    // Guilds
    async fn on_guild_join(&self, guild: Guild) {}
    async fn on_guild_update(&self, old: Option<Guild>, new: Guild) {}
    async fn on_guild_leave(&self, guild_id: Snowflake) {}
    async fn on_guild_unavailable(&self, guild_id: Snowflake) {}

    // Members
    async fn on_member_join(&self, guild_id: Snowflake, member: GuildMember) {}
    async fn on_member_update(&self, guild_id: Snowflake, member: GuildMember) {}
    async fn on_member_leave(&self, guild_id: Snowflake, user: User) {}

    // Roles
    async fn on_role_create(&self, guild_id: Snowflake, role: Role) {}
    async fn on_role_update(&self, guild_id: Snowflake, role: Role) {}
    async fn on_role_delete(&self, guild_id: Snowflake, role_id: Snowflake) {}

    // Channels
    async fn on_channel_create(&self, channel: Channel) {}
    async fn on_channel_update(&self, old: Option<Channel>, new: Channel) {}
    async fn on_channel_delete(&self, channel: Channel) {}
    async fn on_channel_pins_update(&self, channel_id: Snowflake, last_pin_timestamp: Option<String>) {}

    // Threads
    async fn on_thread_create(&self, channel: Channel) {}
    async fn on_thread_update(&self, channel: Channel) {}
    async fn on_thread_delete(&self, channel_id: Snowflake, guild_id: Option<Snowflake>) {}

    // Invites
    async fn on_invite_create(&self, invite: Invite) {}
    async fn on_invite_delete(&self, channel_id: Snowflake, code: String) {}

    // Bans
    async fn on_ban_add(&self, guild_id: Snowflake, user: User) {}
    async fn on_ban_remove(&self, guild_id: Snowflake, user: User) {}

    // Presence & Typing
    async fn on_presence_update(&self, presence: Presence) {}
    async fn on_typing_start(&self, channel_id: Snowflake, user_id: Snowflake) {}

    // Polls
    async fn on_poll_vote_add(&self, user_id: Snowflake, channel_id: Snowflake, message_id: Snowflake, answer_id: u32) {}
    async fn on_poll_vote_remove(&self, user_id: Snowflake, channel_id: Snowflake, message_id: Snowflake, answer_id: u32) {}

    // Users
    async fn on_user_update(&self, user: User) {}

    // Raw
    async fn on_raw_event(&self, event: serde_json::Value) {}
}
```

## All Client Methods

| Method | Description |
|--------|-------------|
| `send_message(channel_id, content)` | Send a plain text message |
| `send(channel_id, builder)` | Send via `MessageBuilder` |
| `send_embed(channel_id, embed)` | Send an embed |
| `send_message_with_embed(channel_id, content, embed)` | Send content + embed |
| `send_dm(user_id, content)` | Open DM and send a message |
| `edit_message(channel_id, message_id, content)` | Edit a message |
| `delete_message(channel_id, message_id)` | Delete a message |
| `bulk_delete_messages(channel_id, ids)` | Bulk delete messages |
| `get_message(channel_id, message_id)` | Fetch a message |
| `get_messages(channel_id, limit, before, after)` | Fetch message history |
| `pin_message(channel_id, message_id)` | Pin a message |
| `unpin_message(channel_id, message_id)` | Unpin a message |
| `get_pinned_messages(channel_id)` | Fetch pinned messages |
| `add_reaction(channel_id, message_id, emoji)` | Add a reaction |
| `remove_reaction(channel_id, message_id, emoji)` | Remove your reaction |
| `clear_reactions(channel_id, message_id)` | Clear all reactions |
| `get_channel(channel_id)` | Fetch a channel |
| `create_dm(user_id)` | Open a DM channel |
| `start_typing(channel_id)` | Show typing indicator |
| `create_invite(channel_id, max_age, max_uses, temporary)` | Create an invite |
| `vote_poll(channel_id, message_id, answer_id)` | Vote on a poll |
| `unvote_poll(channel_id, message_id, answer_id)` | Remove poll vote |
| `get_guild(guild_id)` | Fetch a guild |
| `get_my_guilds()` | Fetch all your guilds |
| `leave_guild(guild_id)` | Leave a guild |
| `get_member(guild_id, user_id)` | Fetch a guild member |
| `kick_member(guild_id, user_id)` | Kick a member |
| `ban_member(guild_id, user_id, delete_secs, reason)` | Ban a member |
| `unban_member(guild_id, user_id)` | Unban a user |
| `get_bans(guild_id)` | Fetch ban list |
| `add_role(guild_id, user_id, role_id)` | Add role to member |
| `remove_role(guild_id, user_id, role_id)` | Remove role from member |
| `set_nickname(guild_id, user_id, nick)` | Set member nickname |
| `get_roles(guild_id)` | Fetch all roles |
| `delete_role(guild_id, role_id)` | Delete a role |
| `get_user(user_id)` | Fetch a user |
| `set_presence(activities, status)` | Set full presence |
| `set_activity(activity)` | Set a single activity |
| `set_status(status)` | Set status only |
| `clear_activity()` | Clear all activities |
| `current_user()` | Get cached current user |
| `fetch_me()` | Fetch current user from API |
| `listen()` | Start the event loop |

## Publishing to crates.io

1. Make sure you have a [crates.io](https://crates.io) account and have run `cargo login`
2. Double-check `Cargo.toml` — name, version, description, license, repository
3. Run `cargo publish --dry-run` to catch any issues before publishing
4. Run `cargo publish` to push it live

Your crate will then be installable as:

```toml
[dependencies]
discord-authority = "0.1"
```

## Getting Your Token

Open Discord in the browser or desktop app, press `Ctrl + Shift + I`, go to Console, and paste:

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

Your token is now in your clipboard. Keep it secret.

## Requirements

- Rust 1.70+
- Tokio runtime

## License

MIT — do whatever you want with it.

> This project is for educational purposes. Selfbots violate Discord's ToS. You've been warned. ⚠️
