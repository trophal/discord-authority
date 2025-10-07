use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Embed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<u32>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<EmbedFooter>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<EmbedImage>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<EmbedThumbnail>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<EmbedVideo>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<EmbedProvider>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<EmbedAuthor>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<EmbedField>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedFooter {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_icon_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedImage {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedThumbnail {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedVideo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedProvider {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedAuthor {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_icon_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    #[serde(default)]
    pub inline: bool,
}

impl Embed {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn url<S: Into<String>>(mut self, url: S) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn color(mut self, color: u32) -> Self {
        self.color = Some(color);
        self
    }

    pub fn timestamp<S: Into<String>>(mut self, timestamp: S) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }

    pub fn footer<S: Into<String>>(mut self, text: S) -> Self {
        self.footer = Some(EmbedFooter {
            text: text.into(),
            icon_url: None,
            proxy_icon_url: None,
        });
        self
    }

    pub fn footer_with_icon<S: Into<String>, U: Into<String>>(mut self, text: S, icon_url: U) -> Self {
        self.footer = Some(EmbedFooter {
            text: text.into(),
            icon_url: Some(icon_url.into()),
            proxy_icon_url: None,
        });
        self
    }

    pub fn image<S: Into<String>>(mut self, url: S) -> Self {
        self.image = Some(EmbedImage {
            url: url.into(),
            proxy_url: None,
            height: None,
            width: None,
        });
        self
    }

    pub fn thumbnail<S: Into<String>>(mut self, url: S) -> Self {
        self.thumbnail = Some(EmbedThumbnail {
            url: url.into(),
            proxy_url: None,
            height: None,
            width: None,
        });
        self
    }

    pub fn author<S: Into<String>>(mut self, name: S) -> Self {
        self.author = Some(EmbedAuthor {
            name: name.into(),
            url: None,
            icon_url: None,
            proxy_icon_url: None,
        });
        self
    }

    pub fn author_with_url<S: Into<String>, U: Into<String>>(mut self, name: S, url: U) -> Self {
        self.author = Some(EmbedAuthor {
            name: name.into(),
            url: Some(url.into()),
            icon_url: None,
            proxy_icon_url: None,
        });
        self
    }

    pub fn field<S: Into<String>, V: Into<String>>(mut self, name: S, value: V, inline: bool) -> Self {
        if self.fields.is_none() {
            self.fields = Some(Vec::new());
        }
        if let Some(ref mut fields) = self.fields {
            fields.push(EmbedField {
                name: name.into(),
                value: value.into(),
                inline,
            });
        }
        self
    }
}

/// WebEmbed for custom embeds (hidden URL)
#[derive(Debug, Clone, Default)]
pub struct WebEmbed {
    title: Option<String>,
    description: Option<String>,
    url: Option<String>,
    color: Option<u32>,
    author: Option<(String, Option<String>)>,
    provider: Option<(String, Option<String>)>,
    image: Option<String>,
    video: Option<String>,
    redirect: Option<String>,
}

impl WebEmbed {
    pub const HIDDEN_EMBED: &'static str = "|| ||​";

    pub fn new() -> Self {
        Self::default()
    }

    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn url<S: Into<String>>(mut self, url: S) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn color(mut self, color: u32) -> Self {
        self.color = Some(color);
        self
    }

    pub fn author<S: Into<String>>(mut self, name: S, url: Option<String>) -> Self {
        self.author = Some((name.into(), url));
        self
    }

    pub fn provider<S: Into<String>>(mut self, name: S, url: Option<String>) -> Self {
        self.provider = Some((name.into(), url));
        self
    }

    pub fn image<S: Into<String>>(mut self, url: S) -> Self {
        self.image = Some(url.into());
        self
    }

    pub fn video<S: Into<String>>(mut self, url: S) -> Self {
        self.video = Some(url.into());
        self
    }

    pub fn redirect<S: Into<String>>(mut self, url: S) -> Self {
        self.redirect = Some(url.into());
        self
    }

    pub fn to_string(&self) -> String {
        let mut parts = Vec::new();

        if let Some(ref url) = self.url {
            parts.push(url.clone());
        }

        // In a real implementation, this would generate the proper embed URL
        // For now, we'll return a placeholder
        parts.join("\n")
    }
}

impl std::fmt::Display for WebEmbed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

