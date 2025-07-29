use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};



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
#[derive(Debug, Clone, Deserialize)]
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

// RSS抓取进度事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RssFetchProgress {
    pub feed_id: String,
    pub feed_title: String,
    pub total_articles: u32,
    pub fetched_articles: u32,
    pub current_article_title: Option<String>,
    pub status: RssFetchStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RssFetchStatus {
    Started,
    InProgress,
    Completed,
    Failed(String),
}

// RSS文章抓取事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RssArticleFetched {
    pub feed_id: String,
    pub article: RssArticle,
}
