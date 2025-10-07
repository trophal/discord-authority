use serde::{Deserialize, Serialize};
use crate::utils::Snowflake;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Presence {
    pub user: serde_json::Value,
    pub guild_id: Option<Snowflake>,
    pub status: String,
    pub activities: Vec<Activity>,
    pub client_status: ClientStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desktop: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub name: String,
    #[serde(rename = "type")]
    pub activity_type: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<ActivityTimestamps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<ActivityEmoji>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party: Option<ActivityParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<ActivityAssets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<ActivitySecrets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityTimestamps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityEmoji {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityParty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<[u32; 2]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityAssets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivitySecrets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spectate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<String>,
}

/// Activity types
pub mod activity_types {
    pub const PLAYING: u8 = 0;
    pub const STREAMING: u8 = 1;
    pub const LISTENING: u8 = 2;
    pub const WATCHING: u8 = 3;
    pub const CUSTOM: u8 = 4;
    pub const COMPETING: u8 = 5;
}

/// Custom Status
#[derive(Debug, Clone)]
pub struct CustomStatus {
    state: Option<String>,
    emoji: Option<ActivityEmoji>,
}

impl CustomStatus {
    pub fn new() -> Self {
        Self {
            state: None,
            emoji: None,
        }
    }

    pub fn state<S: Into<String>>(mut self, state: S) -> Self {
        self.state = Some(state.into());
        self
    }

    pub fn emoji<S: Into<String>>(mut self, emoji: S) -> Self {
        self.emoji = Some(ActivityEmoji {
            name: emoji.into(),
            id: None,
            animated: None,
        });
        self
    }

    pub fn to_activity(&self) -> Activity {
        Activity {
            name: "Custom Status".to_string(),
            activity_type: activity_types::CUSTOM,
            url: None,
            created_at: Some(crate::utils::now()),
            timestamps: None,
            application_id: None,
            details: None,
            state: self.state.clone(),
            emoji: self.emoji.clone(),
            party: None,
            assets: None,
            secrets: None,
            instance: None,
            flags: None,
            buttons: None,
            metadata: None,
        }
    }
}

impl Default for CustomStatus {
    fn default() -> Self {
        Self::new()
    }
}

/// Rich Presence
#[derive(Debug, Clone)]
pub struct RichPresence {
    application_id: Option<Snowflake>,
    name: String,
    details: Option<String>,
    state: Option<String>,
    activity_type: u8,
    url: Option<String>,
    timestamps: Option<ActivityTimestamps>,
    assets: Option<ActivityAssets>,
    party: Option<ActivityParty>,
    buttons: Vec<String>,
    metadata: Option<serde_json::Value>,
}

impl RichPresence {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            application_id: None,
            name: name.into(),
            details: None,
            state: None,
            activity_type: activity_types::PLAYING,
            url: None,
            timestamps: None,
            assets: None,
            party: None,
            buttons: Vec::new(),
            metadata: None,
        }
    }

    pub fn application_id<S: Into<String>>(mut self, id: S) -> Self {
        self.application_id = Some(Snowflake::from(id.into()));
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = name.into();
        self
    }

    pub fn details<S: Into<String>>(mut self, details: S) -> Self {
        self.details = Some(details.into());
        self
    }

    pub fn state<S: Into<String>>(mut self, state: S) -> Self {
        self.state = Some(state.into());
        self
    }

    pub fn activity_type(mut self, activity_type: u8) -> Self {
        self.activity_type = activity_type;
        self
    }

    pub fn url<S: Into<String>>(mut self, url: S) -> Self {
        let url_str = url.into();
        self.url = Some(url_str.clone());
        // If URL is set, automatically change to STREAMING
        if !url_str.is_empty() {
            self.activity_type = activity_types::STREAMING;
        }
        self
    }

    pub fn start_timestamp(mut self, timestamp: u64) -> Self {
        if let Some(ref mut ts) = self.timestamps {
            ts.start = Some(timestamp);
        } else {
            self.timestamps = Some(ActivityTimestamps {
                start: Some(timestamp),
                end: None,
            });
        }
        self
    }

    pub fn end_timestamp(mut self, timestamp: u64) -> Self {
        if let Some(ref mut ts) = self.timestamps {
            ts.end = Some(timestamp);
        } else {
            self.timestamps = Some(ActivityTimestamps {
                start: None,
                end: Some(timestamp),
            });
        }
        self
    }

    pub fn large_image<S: Into<String>>(mut self, image: S) -> Self {
        if let Some(ref mut assets) = self.assets {
            assets.large_image = Some(image.into());
        } else {
            self.assets = Some(ActivityAssets {
                large_image: Some(image.into()),
                large_text: None,
                small_image: None,
                small_text: None,
            });
        }
        self
    }

    pub fn large_text<S: Into<String>>(mut self, text: S) -> Self {
        if let Some(ref mut assets) = self.assets {
            assets.large_text = Some(text.into());
        } else {
            self.assets = Some(ActivityAssets {
                large_image: None,
                large_text: Some(text.into()),
                small_image: None,
                small_text: None,
            });
        }
        self
    }

    pub fn small_image<S: Into<String>>(mut self, image: S) -> Self {
        if let Some(ref mut assets) = self.assets {
            assets.small_image = Some(image.into());
        } else {
            self.assets = Some(ActivityAssets {
                large_image: None,
                large_text: None,
                small_image: Some(image.into()),
                small_text: None,
            });
        }
        self
    }

    pub fn small_text<S: Into<String>>(mut self, text: S) -> Self {
        if let Some(ref mut assets) = self.assets {
            assets.small_text = Some(text.into());
        } else {
            self.assets = Some(ActivityAssets {
                large_image: None,
                large_text: None,
                small_image: None,
                small_text: Some(text.into()),
            });
        }
        self
    }

    pub fn party(mut self, current: u32, max: u32) -> Self {
        self.party = Some(ActivityParty {
            id: None,
            size: Some([current, max]),
        });
        self
    }

    pub fn add_button<S: Into<String>>(mut self, label: S, url: S) -> Self {
        self.buttons.push(label.into());
        if self.metadata.is_none() {
            self.metadata = Some(serde_json::json!({
                "button_urls": [url.into()]
            }));
        } else if let Some(serde_json::Value::Object(ref mut map)) = self.metadata {
            if let Some(serde_json::Value::Array(ref mut urls)) = map.get_mut("button_urls") {
                urls.push(serde_json::json!(url.into()));
            } else {
                map.insert("button_urls".to_string(), serde_json::json!([url.into()]));
            }
        }
        self
    }

    pub fn to_activity(&self) -> Activity {
        Activity {
            name: self.name.clone(),
            activity_type: self.activity_type,
            url: self.url.clone(),
            created_at: Some(crate::utils::now()),
            timestamps: self.timestamps.clone(),
            application_id: self.application_id,
            details: self.details.clone(),
            state: self.state.clone(),
            emoji: None,
            party: self.party.clone(),
            assets: self.assets.clone(),
            secrets: None,
            instance: None,
            flags: None,
            buttons: if self.buttons.is_empty() { None } else { Some(self.buttons.clone()) },
            metadata: self.metadata.clone(),
        }
    }
}

/// Spotify RPC
#[derive(Debug, Clone)]
pub struct SpotifyRPC {
    details: Option<String>,
    state: Option<String>,
    timestamps: Option<ActivityTimestamps>,
    assets: Option<ActivityAssets>,
    party_id: Option<String>,
    sync_id: Option<String>,
    flags: u32,
}

impl SpotifyRPC {
    pub fn new() -> Self {
        Self {
            details: None,
            state: None,
            timestamps: None,
            assets: None,
            party_id: Some("spotify:1".to_string()),
            sync_id: None,
            flags: 48, // Spotify flag
        }
    }

    pub fn details<S: Into<String>>(mut self, details: S) -> Self {
        self.details = Some(details.into());
        self
    }

    pub fn state<S: Into<String>>(mut self, state: S) -> Self {
        self.state = Some(state.into());
        self
    }

    pub fn start_timestamp(mut self, timestamp: u64) -> Self {
        if let Some(ref mut ts) = self.timestamps {
            ts.start = Some(timestamp);
        } else {
            self.timestamps = Some(ActivityTimestamps {
                start: Some(timestamp),
                end: None,
            });
        }
        self
    }

    pub fn end_timestamp(mut self, timestamp: u64) -> Self {
        if let Some(ref mut ts) = self.timestamps {
            ts.end = Some(timestamp);
        } else {
            self.timestamps = Some(ActivityTimestamps {
                start: None,
                end: Some(timestamp),
            });
        }
        self
    }

    pub fn large_image<S: Into<String>>(mut self, image: S) -> Self {
        if let Some(ref mut assets) = self.assets {
            assets.large_image = Some(image.into());
        } else {
            self.assets = Some(ActivityAssets {
                large_image: Some(image.into()),
                large_text: None,
                small_image: None,
                small_text: None,
            });
        }
        self
    }

    pub fn large_text<S: Into<String>>(mut self, text: S) -> Self {
        if let Some(ref mut assets) = self.assets {
            assets.large_text = Some(text.into());
        } else {
            self.assets = Some(ActivityAssets {
                large_image: None,
                large_text: Some(text.into()),
                small_image: None,
                small_text: None,
            });
        }
        self
    }

    pub fn small_image<S: Into<String>>(mut self, image: S) -> Self {
        if let Some(ref mut assets) = self.assets {
            assets.small_image = Some(image.into());
        } else {
            self.assets = Some(ActivityAssets {
                large_image: None,
                large_text: None,
                small_image: Some(image.into()),
                small_text: None,
            });
        }
        self
    }

    pub fn small_text<S: Into<String>>(mut self, text: S) -> Self {
        if let Some(ref mut assets) = self.assets {
            assets.small_text = Some(text.into());
        } else {
            self.assets = Some(ActivityAssets {
                large_image: None,
                large_text: None,
                small_image: None,
                small_text: Some(text.into()),
            });
        }
        self
    }

    pub fn song_id<S: Into<String>>(mut self, id: S) -> Self {
        self.sync_id = Some(id.into());
        self
    }

    pub fn to_activity(&self) -> Activity {
        Activity {
            name: "Spotify".to_string(),
            activity_type: activity_types::LISTENING,
            url: None,
            created_at: Some(crate::utils::now()),
            timestamps: self.timestamps.clone(),
            application_id: Some(Snowflake::from("2648254565127794692")), // Spotify App ID
            details: self.details.clone(),
            state: self.state.clone(),
            emoji: None,
            party: self.party_id.as_ref().map(|id| ActivityParty {
                id: Some(id.clone()),
                size: None,
            }),
            assets: self.assets.clone(),
            secrets: None,
            instance: None,
            flags: Some(self.flags),
            buttons: None,
            metadata: self.sync_id.as_ref().map(|id| serde_json::json!({
                "context_uri": format!("spotify:track:{}", id)
            })),
        }
    }
}

impl Default for SpotifyRPC {
    fn default() -> Self {
        Self::new()
    }
}

