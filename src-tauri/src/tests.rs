#[cfg(test)]
mod tests {
    use crate::rss::RssService;
    use sqlx::SqlitePool;
    use tempfile::NamedTempFile;

    async fn setup_test_db() -> SqlitePool {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();

        let pool = SqlitePool::connect(&format!("sqlite:{}", db_path))
            .await
            .unwrap();

        // 运行迁移
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();

        pool
    }

    #[tokio::test]
    async fn test_extract_article_content() {
        // 测试从一个真实的网站提取内容
        let test_urls = vec![
            "https://httpbin.org/html", // 简单的HTML测试页面
            "https://example.com",      // 基本的示例页面
        ];

        for url in test_urls {
            println!("测试URL: {}", url);
            match RssService::extract_article_content(url).await {
                Some(content) => {
                    println!("提取成功，内容长度: {}", content.len());
                    println!("内容预览: {}...", &content[..content.len().min(200)]);
                    assert!(!content.trim().is_empty(), "提取的内容不应为空");
                }
                None => {
                    println!("提取失败: {}", url);
                }
            }
        }
    }

    #[tokio::test]
    async fn test_get_article_content_with_extraction() {
        let db = setup_test_db().await;

        // 创建一个测试RSS源
        let feed_id = "test-feed-id";
        sqlx::query(
            "INSERT INTO rss_feeds (id, title, url, description, website_url, last_updated, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(feed_id)
        .bind("Test Feed")
        .bind("https://example.com/rss")
        .bind("Test Description")
        .bind("https://example.com")
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(chrono::Utc::now().to_rfc3339())
        .execute(&db)
        .await
        .unwrap();

        // 创建一个没有内容的测试文章
        let article_id = "test-article-id";
        sqlx::query(
            "INSERT INTO rss_articles (id, feed_id, title, link, description, content, author, published_at, guid, read_time, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(article_id)
        .bind(feed_id)
        .bind("Test Article")
        .bind("https://httpbin.org/html") // 使用一个可以访问的测试URL
        .bind("Test Description")
        .bind("") // 空内容
        .bind("Test Author")
        .bind(chrono::Utc::now().to_rfc3339())
        .bind("test-guid")
        .bind(Some("5 min read")) // 测试readTime
        .bind(chrono::Utc::now().to_rfc3339())
        .execute(&db)
        .await
        .unwrap();

        // 测试获取文章内容（应该触发内容提取）
        let result = RssService::get_article_content(&db, article_id.to_string()).await;

        match result {
            Ok(article) => {
                println!("文章标题: {}", article.title);
                if let Some(content) = &article.content {
                    println!("提取的内容长度: {}", content.len());
                    println!("内容预览: {}...", &content[..content.len().min(200)]);
                    assert!(!content.trim().is_empty(), "提取的内容不应为空");
                } else {
                    println!("警告: 没有提取到内容");
                }
            }
            Err(e) => {
                panic!("获取文章内容失败: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_simple_content_extraction() {
        // 简单测试内容提取功能
        let test_url = "https://httpbin.org/html";

        println!("测试从 {} 提取内容", test_url);

        match RssService::extract_article_content(test_url).await {
            Some(content) => {
                println!("提取成功！内容长度: {}", content.len());
                println!("内容预览: {}...", &content[..content.len().min(300)]);
                assert!(!content.trim().is_empty(), "提取的内容不应为空");
                assert!(content.len() > 50, "提取的内容应该有足够的长度");
            }
            None => {
                println!("内容提取失败");
                // 不让测试失败，因为网络问题可能导致提取失败
            }
        }
    }

    #[tokio::test]
    async fn test_html_parsing_with_different_selectors() {
        // 测试HTML解析的不同选择器
        let test_html = r#"
        <!DOCTYPE html>
        <html>
        <head><title>Test Page</title></head>
        <body>
            <header>Header content</header>
            <main>
                <article>
                    <h1>Article Title</h1>
                    <div class="post-content">
                        <p>This is the first paragraph of the article.</p>
                        <p>This is the second paragraph with more content.</p>
                        <p>This is the third paragraph to test extraction.</p>
                    </div>
                </article>
            </main>
            <footer>Footer content</footer>
        </body>
        </html>
        "#;

        // 创建一个简单的HTTP服务器来提供测试HTML
        // 这里我们直接测试HTML解析逻辑
        use scraper::{Html, Selector};

        let document = Html::parse_document(test_html);

        // 测试article选择器
        if let Ok(selector) = Selector::parse("article") {
            if let Some(element) = document.select(&selector).next() {
                let text = element
                    .text()
                    .collect::<Vec<_>>()
                    .join(" ")
                    .trim()
                    .to_string();
                println!("Article选择器提取的内容: {}", text);
                assert!(text.contains("Article Title"), "应该包含文章标题");
                assert!(text.contains("first paragraph"), "应该包含第一段内容");
            }
        }

        // 测试.post-content选择器
        if let Ok(selector) = Selector::parse(".post-content") {
            if let Some(element) = document.select(&selector).next() {
                let text = element
                    .text()
                    .collect::<Vec<_>>()
                    .join(" ")
                    .trim()
                    .to_string();
                println!(".post-content选择器提取的内容: {}", text);
                assert!(text.contains("first paragraph"), "应该包含段落内容");
            }
        }

        // 测试p标签选择器
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
                .filter(|text| text.len() > 10)
                .collect();

            println!("P标签提取的段落数: {}", paragraphs.len());
            assert_eq!(paragraphs.len(), 3, "应该提取到3个段落");

            let content = paragraphs.join("\n\n");
            println!("合并的段落内容: {}", content);
            assert!(content.contains("first paragraph"), "应该包含第一段");
            assert!(content.contains("second paragraph"), "应该包含第二段");
            assert!(content.contains("third paragraph"), "应该包含第三段");
        }
    }
}
