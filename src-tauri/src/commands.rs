use crate::error::AppResult;
use crate::models::{AddFeedRequest, AppState, RssArticle, RssFeed, UpdateArticleRequest, RssFetchProgress, RssFetchStatus};
use crate::rss::RssService;
use tauri::{State, AppHandle, Emitter};
use tokio::task;



// RSS相关命令函数

/// 添加RSS源（同步版本，立即返回feed信息）
#[tauri::command]
pub async fn add_rss_feed_sync(
    state: State<'_, AppState>,
    request: AddFeedRequest,
) -> AppResult<RssFeed> {
    RssService::add_feed_sync(&state.db, request).await
}

/// 添加RSS源（异步版本，后台抓取文章）
#[tauri::command]
pub async fn add_rss_feed_async(
    app_handle: AppHandle,
    state: State<'_, AppState>,
    request: AddFeedRequest,
) -> AppResult<RssFeed> {
    // 首先创建RSS源记录

    let request_clone = request.clone();
    let feed = RssService::add_feed_sync(&state.db, request_clone).await?;
    
    // 克隆必要的数据用于异步任务
    let db = state.db.clone();
    let feed_id = feed.id.clone();
    let feed_title = feed.title.clone();
    let url = request.url.clone();
    let app_handle_clone = app_handle.clone();
    
    // 启动异步任务抓取文章
    task::spawn(async move {
        // 发送开始抓取事件
        let progress = RssFetchProgress {
            feed_id: feed_id.clone(),
            feed_title: feed_title.clone(),
            total_articles: 0,
            fetched_articles: 0,
            current_article_title: None,
            status: RssFetchStatus::Started,
        };
        let _ = app_handle_clone.emit("rss-fetch-progress", &progress);
        
        // 执行异步抓取
        match RssService::fetch_articles_async(&db, &feed_id, &url, &app_handle_clone).await {
            Ok(_) => {
                let progress = RssFetchProgress {
                    feed_id: feed_id.clone(),
                    feed_title: feed_title.clone(),
                    total_articles: 0,
                    fetched_articles: 0,
                    current_article_title: None,
                    status: RssFetchStatus::Completed,
                };
                let _ = app_handle_clone.emit("rss-fetch-progress", &progress);
            }
            Err(e) => {
                let progress = RssFetchProgress {
                    feed_id: feed_id.clone(),
                    feed_title: feed_title.clone(),
                    total_articles: 0,
                    fetched_articles: 0,
                    current_article_title: None,
                    status: RssFetchStatus::Failed(e.to_string()),
                };
                let _ = app_handle_clone.emit("rss-fetch-progress", &progress);
            }
        }
    });
    
    Ok(feed)
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

/// 获取单篇文章详细内容
#[tauri::command]
pub async fn get_article_content(
    state: State<'_, AppState>,
    article_id: String,
) -> AppResult<RssArticle> {
    RssService::get_article_content(&state.db, article_id).await
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

/// 获取统计信息
#[tauri::command]
pub async fn get_statistics(state: State<'_, AppState>) -> AppResult<serde_json::Value> {
    RssService::get_statistics(&state.db).await
}

/// 保留原有的greet函数用于基本测试
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
