use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ForumThreadBundle {
    pub thread: ForumThread,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ForumThread {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub forum_id: Option<u64>,
    #[serde(default)]
    pub posts: Option<u64>,
    #[serde(default)]
    pub views: Option<u64>,
    #[serde(default)]
    pub rating: Option<f64>,
    #[serde(default)]
    pub is_locked: Option<bool>,
    #[serde(default)]
    pub is_sticky: Option<bool>,
    #[serde(default)]
    pub has_poll: Option<bool>,
    #[serde(default)]
    pub first_post_time: Option<u64>,
    #[serde(default)]
    pub last_post_time: Option<u64>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub content_raw: Option<String>,
    #[serde(default)]
    pub author: Option<ForumUserSummary>,
    #[serde(default)]
    pub last_poster: Option<ForumUserSummary>,
    #[serde(default)]
    pub poll: Option<ForumPoll>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ForumUserSummary {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub karma: Option<i64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ForumPoll {
    #[serde(default)]
    pub question: Option<String>,
    #[serde(default)]
    pub answers_count: Option<u64>,
    #[serde(default)]
    pub answers: Vec<ForumPollAnswer>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ForumPollAnswer {
    #[serde(default)]
    pub answer: Option<String>,
    #[serde(default)]
    pub votes: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}
