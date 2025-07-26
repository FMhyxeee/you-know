use crate::error::AppResult;
use crate::models::{AddFeedRequest, AppState, RssArticle, RssFeed, UpdateArticleRequest};
use crate::rss::RssService;
use chrono::Utc;
use sqlx::Row;
use tauri::State;
use uuid::Uuid;

// 技术验证：数据库连接测试
#[tauri::command]
pub async fn test_database_connection(state: State<'_, AppState>) -> AppResult<String> {
    let db = &state.db;

    // 测试数据库连接
    let row = sqlx::query("SELECT 'Database connection successful' as message")
        .fetch_one(db)
        .await?;

    let message: String = row.get("message");
    Ok(message)
}

// 技术验证：创建表结构
#[tauri::command]
pub async fn create_tables(state: State<'_, AppState>) -> AppResult<String> {
    let db = &state.db;

    // 创建conversations表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS conversations (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            model_provider TEXT NOT NULL,
            model_name TEXT NOT NULL,
            system_prompt TEXT,
            temperature REAL DEFAULT 0.7,
            max_tokens INTEGER DEFAULT 2048,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(db)
    .await?;

    // 创建messages表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS messages (
            id TEXT PRIMARY KEY,
            conversation_id TEXT NOT NULL,
            role TEXT NOT NULL,
            content TEXT NOT NULL,
            token_count INTEGER,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (conversation_id) REFERENCES conversations(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(db)
    .await?;

    // 创建provider_configs表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS provider_configs (
            id TEXT PRIMARY KEY,
            provider_name TEXT NOT NULL UNIQUE,
            api_key TEXT NOT NULL,
            base_url TEXT,
            is_enabled BOOLEAN DEFAULT TRUE,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(db)
    .await?;

    Ok("Tables created successfully".to_string())
}

// 技术验证：基本CRUD操作
#[tauri::command]
pub async fn test_crud_operations(state: State<'_, AppState>) -> AppResult<String> {
    let db = &state.db;

    // 插入测试数据
    let conversation_id = Uuid::new_v4().to_string();
    let now = Utc::now();

    sqlx::query(
        "INSERT INTO conversations (id, title, model_provider, model_name, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&conversation_id)
    .bind("Test Conversation")
    .bind("openai")
    .bind("gpt-4")
    .bind(now.to_rfc3339())
    .bind(now.to_rfc3339())
    .execute(db)
    .await?;

    // 查询测试数据
    let row = sqlx::query("SELECT COUNT(*) as count FROM conversations")
        .fetch_one(db)
        .await?;

    let count: i64 = row.get("count");

    // 清理测试数据
    sqlx::query("DELETE FROM conversations WHERE id = ?")
        .bind(&conversation_id)
        .execute(db)
        .await?;

    Ok(format!(
        "CRUD operations successful. Found {} conversations",
        count
    ))
}

// RSS相关命令函数

/// 添加RSS源
#[tauri::command]
pub async fn add_rss_feed(
    state: State<'_, AppState>,
    request: AddFeedRequest,
) -> AppResult<RssFeed> {
    RssService::add_feed(&state.db, request).await
}

/// 获取所有RSS源
#[tauri::command]
pub async fn get_rss_feeds(state: State<'_, AppState>) -> AppResult<Vec<RssFeed>> {
    RssService::get_feeds(&state.db).await
}

/// 获取文章列表
#[tauri::command]
pub async fn get_articles(
    state: State<'_, AppState>,
    feed_id: Option<String>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> AppResult<Vec<RssArticle>> {
    RssService::get_articles(&state.db, feed_id, limit, offset).await
}

/// 更新文章状态
#[tauri::command]
pub async fn update_article(
    state: State<'_, AppState>,
    request: UpdateArticleRequest,
) -> AppResult<String> {
    RssService::update_article(&state.db, request).await
}

/// 刷新RSS源
#[tauri::command]
pub async fn refresh_rss_feed(state: State<'_, AppState>, feed_id: String) -> AppResult<String> {
    RssService::refresh_feed(&state.db, feed_id).await
}

/// 删除RSS源
#[tauri::command]
pub async fn delete_rss_feed(state: State<'_, AppState>, feed_id: String) -> AppResult<String> {
    RssService::delete_feed(&state.db, feed_id).await
}

/// 保留原有的greet函数用于基本测试
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
