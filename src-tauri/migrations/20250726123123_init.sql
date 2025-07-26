-- Add migration script here
-- RSS源表
CREATE TABLE IF NOT EXISTS rss_feeds (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    url TEXT NOT NULL UNIQUE,
    description TEXT,
    website_url TEXT,
    last_updated TEXT,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);

-- RSS文章表
CREATE TABLE IF NOT EXISTS rss_articles (
    id TEXT PRIMARY KEY,
    feed_id TEXT NOT NULL,
    title TEXT NOT NULL,
    link TEXT,
    description TEXT,
    content TEXT,
    author TEXT,
    published_at TEXT,
    guid TEXT,
    is_read BOOLEAN DEFAULT FALSE,
    is_starred BOOLEAN DEFAULT FALSE,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (feed_id) REFERENCES rss_feeds(id) ON DELETE CASCADE
);

-- 创建索引
CREATE INDEX IF NOT EXISTS idx_rss_articles_feed_id ON rss_articles(feed_id);
CREATE INDEX IF NOT EXISTS idx_rss_articles_published_at ON rss_articles(published_at);
CREATE INDEX IF NOT EXISTS idx_rss_articles_is_read ON rss_articles(is_read);
CREATE INDEX IF NOT EXISTS idx_rss_articles_is_starred ON rss_articles(is_starred);
CREATE UNIQUE INDEX IF NOT EXISTS idx_rss_articles_guid_feed ON rss_articles(guid, feed_id);