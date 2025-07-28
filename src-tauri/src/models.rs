use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// 对话相关数据模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub title: String,
    pub model_provider: String,
    pub model_name: String,
    pub system_prompt: Option<String>,
    pub temperature: f64,
    pub max_tokens: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub conversation_id: String,
    pub role: String,
    pub content: String,
    pub token_count: Option<i32>,
    pub created_at: DateTime<Utc>,
}

// RSS相关数据模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RssFeed {
    pub id: String,
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub website_url: Option<String>,
    pub last_updated: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RssArticle {
    pub id: String,
    pub feed_id: String,
    pub title: String,
    pub link: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub author: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
    pub guid: Option<String>,
    pub is_read: bool,
    pub is_starred: bool,
    pub read_time: Option<String>,
    pub created_at: DateTime<Utc>,
}

// 请求数据模型
#[derive(Debug, Deserialize)]
pub struct AddFeedRequest {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateArticleRequest {
    pub id: String,
    pub is_read: Option<bool>,
    pub is_starred: Option<bool>,
}

// 应用状态
#[derive(Debug)]
pub struct AppState {
    pub db: sqlx::SqlitePool,
}
