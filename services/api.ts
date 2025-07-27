import { invoke } from '@tauri-apps/api/core';

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

// API服务类
export class ApiService {
  // 测试数据库连接
  static async testDatabaseConnection(): Promise<string> {
    return await invoke('test_database_connection');
  }

  // 创建表结构
  static async createTables(): Promise<string> {
    return await invoke('create_tables');
  }

  // 测试CRUD操作
  static async testCrudOperations(): Promise<string> {
    return await invoke('test_crud_operations');
  }

  // RSS源管理
  static async addRssFeed(request: AddFeedRequest): Promise<RssFeed> {
    return await invoke('add_rss_feed', { request });
  }

  static async getRssFeeds(): Promise<RssFeed[]> {
    return await invoke('get_rss_feeds');
  }

  static async deleteRssFeed(feedId: string): Promise<string> {
    return await invoke('delete_rss_feed', { feedId });
  }

  static async refreshRssFeed(feedId: string): Promise<string> {
    return await invoke('refresh_rss_feed', { feedId });
  }

  // 文章管理
  static async getArticles(
    feedId?: string,
    limit?: number,
    offset?: number
  ): Promise<RssArticle[]> {
    return await invoke('get_articles', { feedId, limit, offset });
  }

  static async getArticleContent(articleId: string): Promise<RssArticle> {
    return await invoke('get_article_content', { articleId });
  }

  static async updateArticle(request: UpdateArticleRequest): Promise<string> {
    return await invoke('update_article', { request });
  }

  // 统计信息
  static async getStatistics(): Promise<any> {
    return await invoke('get_statistics');
  }

  // 基本测试函数
  static async greet(name: string): Promise<string> {
    return await invoke('greet', { name });
  }
}