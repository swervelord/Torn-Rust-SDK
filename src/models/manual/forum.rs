use std::collections::BTreeMap;

use serde::Deserialize;

use super::common::PaginatedMetadata;

#[derive(Debug, Clone, Deserialize)]
pub struct ForumCategoriesBundle {
    #[serde(default)]
    pub categories: Vec<ForumCategory>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ForumLookupBundle {
    #[serde(default)]
    pub selections: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ForumThreadsBundle {
    #[serde(default)]
    pub threads: Vec<ForumThread>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ForumThreadBundle {
    pub thread: ForumThread,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ForumPostsBundle {
    #[serde(default)]
    pub posts: Vec<ForumPost>,
    #[serde(default)]
    pub _metadata: Option<PaginatedMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ForumTimestampBundle {
    #[serde(default)]
    pub timestamp: Option<u64>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ForumCategory {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub acronym: Option<String>,
    #[serde(default)]
    pub threads: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    pub rating: Option<i64>,
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
    pub poll: Option<ForumPoll>,
    #[serde(default)]
    pub author: Option<ForumUserSummary>,
    #[serde(default)]
    pub last_poster: Option<ForumUserSummary>,
    #[serde(default)]
    pub new_posts: Option<u64>,
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ForumPost {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub thread_id: Option<u64>,
    #[serde(default)]
    pub author: Option<ForumUserSummary>,
    #[serde(default)]
    pub is_legacy: Option<bool>,
    #[serde(default)]
    pub is_topic: Option<bool>,
    #[serde(default)]
    pub is_edited: Option<bool>,
    #[serde(default)]
    pub is_pinned: Option<bool>,
    #[serde(default)]
    pub created_time: Option<u64>,
    #[serde(default)]
    pub edited_by: Option<u64>,
    #[serde(default)]
    pub has_quote: Option<bool>,
    #[serde(default)]
    pub quoted_post_id: Option<u64>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub likes: Option<u64>,
    #[serde(default)]
    pub dislikes: Option<u64>,
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
