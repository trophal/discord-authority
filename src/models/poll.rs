use serde::{Deserialize, Serialize};
use crate::utils::Snowflake;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Poll {
    pub question: PollQuestion,
    pub answers: Vec<PollAnswer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_multiselect: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout_type: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<PollResults>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollQuestion {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollAnswer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer_id: Option<u32>,
    pub poll_media: PollMedia,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollMedia {
    pub text: Option<String>,
    pub emoji: Option<PollEmoji>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollEmoji {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollResults {
    pub is_finalized: bool,
    pub answer_counts: Vec<AnswerCount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerCount {
    pub id: u32,
    pub count: u32,
    pub me_voted: bool,
}

impl Poll {
    pub fn new<S: Into<String>>(question: S) -> Self {
        Self {
            question: PollQuestion {
                text: question.into(),
            },
            answers: Vec::new(),
            expiry: None,
            allow_multiselect: None,
            layout_type: Some(1),
            results: None,
        }
    }

    pub fn add_answer<S: Into<String>>(mut self, text: S, emoji: Option<String>) -> Self {
        let answer = PollAnswer {
            answer_id: None,
            poll_media: PollMedia {
                text: Some(text.into()),
                emoji: emoji.map(|e| PollEmoji {
                    id: None,
                    name: Some(e),
                    animated: None,
                }),
            },
        };
        self.answers.push(answer);
        self
    }

    pub fn duration_hours(mut self, hours: u32) -> Self {
        let duration = chrono::Duration::hours(hours as i64);
        let expiry = chrono::Utc::now() + duration;
        self.expiry = Some(expiry.to_rfc3339());
        self
    }

    pub fn allow_multiselect(mut self, allow: bool) -> Self {
        self.allow_multiselect = Some(allow);
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct PollBuilder {
    question: Option<String>,
    answers: Vec<(String, Option<String>)>,
    duration: Option<u32>,
    allow_multiselect: bool,
}

impl PollBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn question<S: Into<String>>(mut self, question: S) -> Self {
        self.question = Some(question.into());
        self
    }

    pub fn answer<S: Into<String>>(mut self, text: S, emoji: Option<String>) -> Self {
        self.answers.push((text.into(), emoji));
        self
    }

    pub fn duration(mut self, hours: u32) -> Self {
        self.duration = Some(hours);
        self
    }

    pub fn allow_multiselect(mut self, allow: bool) -> Self {
        self.allow_multiselect = allow;
        self
    }

    pub fn build(self) -> Result<Poll, String> {
        let question = self.question.ok_or("Question is required")?;
        
        if self.answers.is_empty() {
            return Err("At least one answer is required".to_string());
        }

        let mut poll = Poll::new(question);
        
        for (text, emoji) in self.answers {
            poll = poll.add_answer(text, emoji);
        }

        if let Some(duration) = self.duration {
            poll = poll.duration_hours(duration);
        }

        poll = poll.allow_multiselect(self.allow_multiselect);

        Ok(poll)
    }
}

