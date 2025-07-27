use crate::error::{AppError, AppResult};
use crate::models::{AddFeedRequest, RssArticle, RssFeed, UpdateArticleRequest};
use chrono::{DateTime, Utc};
use feed_rs::parser;
use readability::extractor;
use reqwest;
use scraper::{Html, Selector};
use sqlx::{Row, SqlitePool};
use url::Url;
use uuid::Uuid;

/// RSS服务结构体
pub struct RssService;

impl RssService {
    /// 添加RSS源
    pub async fn add_feed(db: &SqlitePool, request: AddFeedRequest) -> AppResult<RssFeed> {
        // 验证URL格式
        let url = Url::parse(&request.url).map_err(|_| AppError::invalid_rss_url(&request.url))?;

        // 获取RSS内容并解析
        let response = reqwest::get(url.as_str()).await?;
        let content = response.text().await?;

        let feed = parser::parse(content.as_bytes())?;

        let feed_id = Uuid::new_v4().to_string();
        let now = Utc::now();

        let title = feed
            .title
            .map(|t| t.content)
            .unwrap_or_else(|| "Untitled Feed".to_string());
        let description = feed.description.map(|d| d.content);
        let website_url = feed.links.first().map(|l| l.href.clone());

        // 插入RSS源到数据库
        sqlx::query(
            "INSERT INTO rss_feeds (id, title, url, description, website_url, last_updated, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&feed_id)
        .bind(&title)
        .bind(&request.url)
        .bind(&description)
        .bind(&website_url)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(db)
        .await?;

        // 解析并保存文章
        Self::save_articles(db, &feed_id, &feed.entries, &now).await?;

        Ok(RssFeed {
            id: feed_id,
            title,
            url: request.url,
            description,
            website_url,
            last_updated: Some(now),
            is_active: true,
            created_at: now,
            updated_at: now,
        })
    }

    /// 获取所有RSS源
    pub async fn get_feeds(db: &SqlitePool) -> AppResult<Vec<RssFeed>> {
        let rows = sqlx::query(
            "SELECT id, title, url, description, website_url, last_updated, is_active, created_at, updated_at FROM rss_feeds ORDER BY created_at DESC"
        )
        .fetch_all(db)
        .await?;

        let mut feeds = Vec::new();
        for row in rows {
            let created_at_str: String = row.get("created_at");
            let updated_at_str: String = row.get("updated_at");
            let last_updated_str: Option<String> = row.get("last_updated");

            feeds.push(RssFeed {
                id: row.get("id"),
                title: row.get("title"),
                url: row.get("url"),
                description: row.get("description"),
                website_url: row.get("website_url"),
                last_updated: last_updated_str.and_then(|s| {
                    DateTime::parse_from_rfc3339(&s)
                        .ok()
                        .map(|dt| dt.with_timezone(&Utc))
                }),
                is_active: row.get("is_active"),
                created_at: DateTime::parse_from_rfc3339(&created_at_str)
                    .unwrap()
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&updated_at_str)
                    .unwrap()
                    .with_timezone(&Utc),
            });
        }

        Ok(feeds)
    }

    /// 获取文章列表
    pub async fn get_articles(
        db: &SqlitePool,
        feed_id: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> AppResult<Vec<RssArticle>> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);

        let query = if let Some(feed_id) = feed_id {
            sqlx::query(
                "SELECT id, feed_id, title, link, description, content, author, published_at, guid, is_read, is_starred, created_at FROM rss_articles WHERE feed_id = ? ORDER BY published_at DESC, created_at DESC LIMIT ? OFFSET ?"
            )
            .bind(feed_id)
            .bind(limit)
            .bind(offset)
        } else {
            sqlx::query(
                "SELECT id, feed_id, title, link, description, content, author, published_at, guid, is_read, is_starred, created_at FROM rss_articles ORDER BY published_at DESC, created_at DESC LIMIT ? OFFSET ?"
            )
            .bind(limit)
            .bind(offset)
        };

        let rows = query.fetch_all(db).await?;

        let mut articles = Vec::new();
        for row in rows {
            let created_at_str: String = row.get("created_at");
            let published_at_str: Option<String> = row.get("published_at");

            articles.push(RssArticle {
                id: row.get("id"),
                feed_id: row.get("feed_id"),
                title: row.get("title"),
                link: row.get("link"),
                description: row.get("description"),
                content: row.get("content"),
                author: row.get("author"),
                published_at: published_at_str.and_then(|s| {
                    DateTime::parse_from_rfc3339(&s)
                        .ok()
                        .map(|dt| dt.with_timezone(&Utc))
                }),
                guid: row.get("guid"),
                is_read: row.get("is_read"),
                is_starred: row.get("is_starred"),
                created_at: DateTime::parse_from_rfc3339(&created_at_str)
                    .unwrap()
                    .with_timezone(&Utc),
            });
        }

        Ok(articles)
    }

    /// 获取统计信息
    pub async fn get_statistics(db: &SqlitePool) -> AppResult<serde_json::Value> {
        // 获取总文章数
        let total_articles_row = sqlx::query("SELECT COUNT(*) as count FROM rss_articles")
            .fetch_one(db)
            .await?;
        let total_articles: i64 = total_articles_row.get("count");

        // 获取未读文章数
        let unread_articles_row =
            sqlx::query("SELECT COUNT(*) as count FROM rss_articles WHERE is_read = 0")
                .fetch_one(db)
                .await?;
        let unread_articles: i64 = unread_articles_row.get("count");

        // 获取已收藏文章数
        let starred_articles_row =
            sqlx::query("SELECT COUNT(*) as count FROM rss_articles WHERE is_starred = 1")
                .fetch_one(db)
                .await?;
        let starred_articles: i64 = starred_articles_row.get("count");

        // 获取RSS源数量
        let total_feeds_row =
            sqlx::query("SELECT COUNT(*) as count FROM rss_feeds WHERE is_active = 1")
                .fetch_one(db)
                .await?;
        let total_feeds: i64 = total_feeds_row.get("count");

        // 获取每个RSS源的未读文章数
        let feed_unread_rows = sqlx::query(
            "SELECT f.id, f.title, COUNT(a.id) as unread_count 
             FROM rss_feeds f 
             LEFT JOIN rss_articles a ON f.id = a.feed_id AND a.is_read = 0 
             WHERE f.is_active = 1 
             GROUP BY f.id, f.title",
        )
        .fetch_all(db)
        .await?;

        let mut feed_stats = Vec::new();
        for row in feed_unread_rows {
            feed_stats.push(serde_json::json!({
                "id": row.get::<String, _>("id"),
                "title": row.get::<String, _>("title"),
                "unread_count": row.get::<i64, _>("unread_count")
            }));
        }

        Ok(serde_json::json!({
            "total_articles": total_articles,
            "unread_articles": unread_articles,
            "starred_articles": starred_articles,
            "total_feeds": total_feeds,
            "feed_stats": feed_stats
        }))
    }

    /// 提取HTML内容的主要文本
    pub async fn extract_article_content(url: &str) -> Option<String> {
        println!("[DEBUG] 开始提取文章内容: {}", url);

        // 创建带有用户代理的客户端
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .ok()?;

        // 获取网页内容
        let response = match client.get(url).send().await {
            Ok(resp) => resp,
            Err(e) => {
                println!("[ERROR] 请求失败: {}", e);
                return None;
            }
        };

        let html_content = match response.text().await {
            Ok(content) => content,
            Err(e) => {
                println!("[ERROR] 读取响应内容失败: {}", e);
                return None;
            }
        };

        println!("[DEBUG] 获取到HTML内容，长度: {}", html_content.len());

        // 使用readability提取主要内容
        if let Ok(parsed_url) = url.parse::<url::Url>() {
            match extractor::extract(&mut html_content.as_bytes(), &parsed_url) {
                Ok(product) => {
                    if !product.content.trim().is_empty() {
                        println!(
                            "[DEBUG] Readability提取成功，内容长度: {}",
                            product.content.len()
                        );
                        return Some(product.content);
                    }
                }
                Err(e) => {
                    println!("[DEBUG] Readability提取失败: {}", e);
                }
            }
        }

        // 如果readability失败，使用scraper进行简单的内容提取
        let document = Html::parse_document(&html_content);

        // 尝试常见的文章内容选择器
        let selectors = [
            "article",
            ".post-content",
            ".entry-content",
            ".content",
            "main",
            ".article-body",
            "#content",
            ".post-body",
            ".article-content",
            ".post",
            "[role='main']",
        ];

        for selector_str in &selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(element) = document.select(&selector).next() {
                    let text = element
                        .text()
                        .collect::<Vec<_>>()
                        .join(" ")
                        .trim()
                        .to_string();
                    if text.len() > 100 {
                        // 确保内容足够长
                        println!(
                            "[DEBUG] 使用选择器 '{}' 提取成功，内容长度: {}",
                            selector_str,
                            text.len()
                        );
                        return Some(text);
                    }
                }
            }
        }

        // 最后尝试提取所有p标签的内容
        if let Ok(p_selector) = Selector::parse("p") {
            let paragraphs: Vec<String> = document
                .select(&p_selector)
                .map(|element| {
                    element
                        .text()
                        .collect::<Vec<_>>()
                        .join(" ")
                        .trim()
                        .to_string()
                })
                .filter(|text| text.len() > 20)
                .collect();

            if !paragraphs.is_empty() {
                let content = paragraphs.join("\n\n");
                println!("[DEBUG] 使用p标签提取成功，内容长度: {}", content.len());
                return Some(content);
            }
        }

        println!("[DEBUG] 所有提取方法都失败了");
        None
    }

    /// 获取单篇文章详细内容
    pub async fn get_article_content(db: &SqlitePool, article_id: String) -> AppResult<RssArticle> {
        let row = sqlx::query(
            "SELECT id, feed_id, title, link, description, content, author, published_at, guid, is_read, is_starred, created_at FROM rss_articles WHERE id = ?"
        )
        .bind(&article_id)
        .fetch_one(db)
        .await
        .map_err(|_| AppError::article_not_found(&article_id))?;

        let created_at_str: String = row.get("created_at");
        let published_at_str: Option<String> = row.get("published_at");

        // 如果content为空，尝试从原始链接获取完整内容
        let mut content: Option<String> = row.get("content");
        let link: Option<String> = row.get("link");

        // 如果content为空且有链接，尝试获取完整内容
        if (content.is_none() || content.as_ref().map_or(true, |c| c.trim().is_empty()))
            && link.is_some()
        {
            if let Some(extracted_content) =
                Self::extract_article_content(link.as_ref().unwrap()).await
            {
                content = Some(extracted_content);

                // 将提取的内容保存到数据库中，避免重复提取
                let _ = sqlx::query("UPDATE rss_articles SET content = ? WHERE id = ?")
                    .bind(&content)
                    .bind(&article_id)
                    .execute(db)
                    .await;
            }
        }

        Ok(RssArticle {
            id: row.get("id"),
            feed_id: row.get("feed_id"),
            title: row.get("title"),
            link,
            description: row.get("description"),
            content,
            author: row.get("author"),
            published_at: published_at_str.and_then(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .ok()
                    .map(|dt| dt.with_timezone(&Utc))
            }),
            guid: row.get("guid"),
            is_read: row.get("is_read"),
            is_starred: row.get("is_starred"),
            created_at: DateTime::parse_from_rfc3339(&created_at_str)
                .unwrap()
                .with_timezone(&Utc),
        })
    }

    /// 更新文章状态
    pub async fn update_article(
        db: &SqlitePool,
        request: UpdateArticleRequest,
    ) -> AppResult<String> {
        // 简化的更新方法
        if let Some(is_read) = request.is_read {
            sqlx::query("UPDATE rss_articles SET is_read = ? WHERE id = ?")
                .bind(is_read)
                .bind(&request.id)
                .execute(db)
                .await?;
        }

        if let Some(is_starred) = request.is_starred {
            sqlx::query("UPDATE rss_articles SET is_starred = ? WHERE id = ?")
                .bind(is_starred)
                .bind(&request.id)
                .execute(db)
                .await?;
        }

        Ok("Article updated successfully".to_string())
    }

    /// 刷新RSS源
    pub async fn refresh_feed(db: &SqlitePool, feed_id: String) -> AppResult<String> {
        // 获取RSS源信息
        let row = sqlx::query("SELECT url FROM rss_feeds WHERE id = ?")
            .bind(&feed_id)
            .fetch_one(db)
            .await
            .map_err(|_| AppError::feed_not_found(&feed_id))?;

        let url: String = row.get("url");

        // 获取RSS内容并解析
        let response = reqwest::get(&url).await?;
        let content = response.text().await?;

        let feed = parser::parse(content.as_bytes())?;

        let now = Utc::now();
        let new_articles = Self::save_articles(db, &feed_id, &feed.entries, &now).await?;

        // 更新RSS源的最后更新时间
        sqlx::query("UPDATE rss_feeds SET last_updated = ?, updated_at = ? WHERE id = ?")
            .bind(now.to_rfc3339())
            .bind(now.to_rfc3339())
            .bind(&feed_id)
            .execute(db)
            .await?;

        Ok(format!(
            "Refreshed successfully. {} new articles added.",
            new_articles
        ))
    }

    /// 删除RSS源
    pub async fn delete_feed(db: &SqlitePool, feed_id: String) -> AppResult<String> {
        let result = sqlx::query("DELETE FROM rss_feeds WHERE id = ?")
            .bind(&feed_id)
            .execute(db)
            .await?;

        if result.rows_affected() > 0 {
            Ok("RSS feed deleted successfully".to_string())
        } else {
            Err(AppError::feed_not_found(&feed_id))
        }
    }

    /// 保存文章到数据库
    pub async fn save_articles(
        db: &SqlitePool,
        feed_id: &str,
        entries: &[feed_rs::model::Entry],
        now: &DateTime<Utc>,
    ) -> AppResult<i32> {
        let mut new_articles = 0;

        for entry in entries {
            let article_id = Uuid::new_v4().to_string();
            let article_title = entry
                .title
                .as_ref()
                .map(|t| t.content.clone())
                .unwrap_or_else(|| "Untitled Article".to_string());
            let link = entry.links.first().map(|l| l.href.clone());
            let description = entry.summary.as_ref().map(|s| s.content.clone());
            let mut content = entry
                .content
                .as_ref()
                .map(|c| c.body.clone().unwrap_or_default());
            let author = entry.authors.first().map(|a| a.name.clone());
            let published_at = entry.published.map(|p| p.to_rfc3339());
            let guid = Some(entry.id.clone());

            // 如果RSS中没有完整内容，尝试从链接获取
            if (content.is_none() || content.as_ref().map_or(true, |c| c.trim().is_empty()))
                && link.is_some()
            {
                if let Some(extracted_content) =
                    Self::extract_article_content(link.as_ref().unwrap()).await
                {
                    content = Some(extracted_content);
                }
            }

            let result = sqlx::query(
                "INSERT OR IGNORE INTO rss_articles (id, feed_id, title, link, description, content, author, published_at, guid, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(&article_id)
            .bind(feed_id)
            .bind(&article_title)
            .bind(&link)
            .bind(&description)
            .bind(&content)
            .bind(&author)
            .bind(&published_at)
            .bind(&guid)
            .bind(now.to_rfc3339())
            .execute(db)
            .await?;

            if result.rows_affected() > 0 {
                new_articles += 1;
            }
        }

        Ok(new_articles)
    }
}
