import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

// 数据类型定义
export interface RssFeed {
  id: string;
  title: string;
  url: string;
  description?: string;
  website_url?: string;
  last_updated?: string;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface RssArticle {
  id: string;
  feed_id: string;
  title: string;
  link?: string;
  description?: string;
  content?: string;
  author?: string;
  published_at?: string;
  guid?: string;
  is_read: boolean;
  is_starred: boolean;
  read_time?: string;
  created_at: string;
}

export interface AddFeedRequest {
  url: string;
}

export interface UpdateArticleRequest {
  id: string;
  is_read?: boolean;
  is_starred?: boolean;
}

export interface Statistics {
  total_feeds: number;
  total_articles: number;
  unread_articles: number;
  starred_articles: number;
}

// RSS抓取进度事件
export interface RssFetchProgress {
  feed_id: string;
  feed_title: string;
  total_articles: number;
  fetched_articles: number;
  current_article_title?: string;
  status: 'Started' | 'InProgress' | 'Completed' | { Failed: string };
}

// RSS文章抓取事件
export interface RssArticleFetched {
  feed_id: string;
  article: RssArticle;
}

// API服务类
export class ApiService {


  // RSS源管理
  static async addRssFeedSync(request: AddFeedRequest): Promise<RssFeed> {
    return await invoke("add_rss_feed_sync", { request });
  }

  static async addRssFeedAsync(request: AddFeedRequest): Promise<RssFeed> {
    return await invoke("add_rss_feed_async", { request });
  }

  static async getRssFeeds(): Promise<RssFeed[]> {
    return await invoke("get_rss_feeds");
  }

  static async deleteRssFeed(feedId: string): Promise<string> {
    return await invoke("delete_rss_feed", { feedId });
  }

  static async refreshRssFeed(feedId: string): Promise<string> {
    return await invoke("refresh_rss_feed", { feedId });
  }

  // 文章管理
  static async getArticles(
    feedId?: string,
    limit?: number,
    offset?: number,
  ): Promise<RssArticle[]> {
    return await invoke("get_articles", { feedId, limit, offset });
  }

  static async getArticleContent(articleId: string): Promise<RssArticle> {
    return await invoke("get_article_content", { articleId });
  }

  static async updateArticle(request: UpdateArticleRequest): Promise<string> {
    return await invoke("update_article", { request });
  }

  // 统计信息
  static async getStatistics(): Promise<Statistics> {
    return await invoke("get_statistics");
  }

  // 基本测试函数
  static async greet(name: string): Promise<string> {
    return await invoke("greet", { name });
  }

  // 事件监听
  static async listenToRssFetchProgress(
    callback: (progress: RssFetchProgress) => void
  ): Promise<() => void> {
    return await listen<RssFetchProgress>("rss-fetch-progress", (event) => {
      callback(event.payload);
    });
  }

  static async listenToRssArticleFetched(
    callback: (articleEvent: RssArticleFetched) => void
  ): Promise<() => void> {
    return await listen<RssArticleFetched>("rss-article-fetched", (event) => {
      callback(event.payload);
    });
  }
}
