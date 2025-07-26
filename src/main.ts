import { invoke } from "@tauri-apps/api/core";

// 数据类型定义
interface RssFeed {
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

interface RssArticle {
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

// 全局状态
class AppState {
  feeds: RssFeed[] = [];
  articles: RssArticle[] = [];
  currentFeedId: string | null = null;
  currentArticle: RssArticle | null = null;
  isLoading = false;

  constructor() {
    this.init();
  }

  async init() {
    await this.loadFeeds();
    await this.loadArticles();
    this.setupEventListeners();
  }

  async loadFeeds() {
    try {
      this.feeds = await invoke<RssFeed[]>('get_rss_feeds');
      this.renderFeeds();
      this.updateFeedCounts();
    } catch (error) {
      console.error('Failed to load feeds:', error);
      this.showError('加载RSS源失败');
    }
  }

  async loadArticles(feedId?: string) {
    try {
      this.setLoading(true);
      this.articles = await invoke<RssArticle[]>('get_articles', {
        feedId: feedId || null,
        limit: 100,
        offset: 0
      });
      this.renderArticles();
      this.updateFeedCounts();
    } catch (error) {
      console.error('Failed to load articles:', error);
      this.showError('加载文章失败');
    } finally {
      this.setLoading(false);
    }
  }

  renderFeeds() {
    const container = document.getElementById('feeds-container')!;
    container.innerHTML = '';

    this.feeds.forEach(feed => {
      const feedElement = document.createElement('div');
      feedElement.className = 'feed-item';
      feedElement.dataset.feedId = feed.id;
      
      const unreadCount = this.articles.filter(article => 
        article.feed_id === feed.id && !article.is_read
      ).length;

      feedElement.innerHTML = `
        <span class="feed-title" title="${feed.title}">${feed.title}</span>
        <div class="feed-actions">
          <button class="btn btn-icon refresh-feed" data-feed-id="${feed.id}" title="刷新">↻</button>
          <button class="btn btn-icon delete-feed" data-feed-id="${feed.id}" title="删除">×</button>
        </div>
        ${unreadCount > 0 ? `<span class="feed-count">${unreadCount}</span>` : ''}
      `;

      feedElement.addEventListener('click', (e) => {
        if (!(e.target as Element).closest('.feed-actions')) {
          this.selectFeed(feed.id);
        }
      });

      container.appendChild(feedElement);
    });
  }

  renderArticles() {
    const container = document.getElementById('articles-list')!;
    container.innerHTML = '';

    if (this.articles.length === 0) {
      container.innerHTML = '<div class="no-articles">暂无文章</div>';
      return;
    }

    this.articles.forEach(article => {
      const articleElement = document.createElement('div');
      articleElement.className = `article-item ${!article.is_read ? 'unread' : ''} ${article.is_starred ? 'starred' : ''}`;
      articleElement.dataset.articleId = article.id;

      const publishedDate = article.published_at 
        ? new Date(article.published_at).toLocaleDateString('zh-CN')
        : '';
      
      const feed = this.feeds.find(f => f.id === article.feed_id);
      const feedTitle = feed ? feed.title : '未知来源';

      articleElement.innerHTML = `
        <div class="article-title">${article.title}</div>
        <div class="article-meta">
          <span class="article-source">${feedTitle}</span>
          ${article.author ? `<span class="article-author">${article.author}</span>` : ''}
          ${publishedDate ? `<span class="article-date">${publishedDate}</span>` : ''}
        </div>
        ${article.description ? `<div class="article-description">${article.description}</div>` : ''}
      `;

      articleElement.addEventListener('click', () => {
        this.openArticle(article);
      });

      container.appendChild(articleElement);
    });
  }

  selectFeed(feedId: string) {
    // 更新UI状态
    document.querySelectorAll('.feed-item').forEach(item => {
      item.classList.remove('active');
    });
    
    if (feedId) {
      const feedElement = document.querySelector(`[data-feed-id="${feedId}"]`);
      feedElement?.classList.add('active');
      const feed = this.feeds.find(f => f.id === feedId);
      document.getElementById('current-feed-title')!.textContent = feed?.title || '未知RSS源';
    } else {
      document.querySelector('.all-feeds')?.classList.add('active');
      document.getElementById('current-feed-title')!.textContent = '所有文章';
    }

    this.currentFeedId = feedId;
    this.loadArticles(feedId);
  }

  async openArticle(article: RssArticle) {
    this.currentArticle = article;
    
    // 标记为已读
    if (!article.is_read) {
      await this.updateArticleStatus(article.id, { is_read: true });
    }

    // 更新阅读器内容
    document.getElementById('reader-title')!.textContent = article.title;
    document.getElementById('reader-author')!.textContent = article.author || '';
    document.getElementById('reader-date')!.textContent = article.published_at 
      ? new Date(article.published_at).toLocaleString('zh-CN')
      : '';
    
    const sourceLink = document.getElementById('reader-source')! as HTMLAnchorElement;
    if (article.link) {
      sourceLink.href = article.link;
      sourceLink.textContent = '查看原文';
      sourceLink.style.display = 'inline';
    } else {
      sourceLink.style.display = 'none';
    }

    document.getElementById('reader-body')!.innerHTML = article.content || article.description || '无内容';
    
    // 更新操作按钮状态
    const starBtn = document.getElementById('toggle-star-btn')!;
    starBtn.textContent = article.is_starred ? '★' : '☆';
    starBtn.className = `btn btn-icon ${article.is_starred ? 'starred' : ''}`;
    
    const readBtn = document.getElementById('toggle-read-btn')!;
    readBtn.textContent = article.is_read ? '已读' : '未读';

    const openBtn = document.getElementById('open-original-btn')! as HTMLButtonElement;
    openBtn.disabled = !article.link;

    // 显示阅读器
    document.getElementById('article-reader')!.classList.add('active');
  }

  closeReader() {
    document.getElementById('article-reader')!.classList.remove('active');
    this.currentArticle = null;
  }

  async updateArticleStatus(articleId: string, updates: { is_read?: boolean; is_starred?: boolean }) {
    try {
      await invoke('update_article', {
        request: {
          id: articleId,
          ...updates
        }
      });
      
      // 更新本地状态
      const article = this.articles.find(a => a.id === articleId);
      if (article) {
        if (updates.is_read !== undefined) article.is_read = updates.is_read;
        if (updates.is_starred !== undefined) article.is_starred = updates.is_starred;
      }
      
      this.renderArticles();
      this.updateFeedCounts();
    } catch (error) {
      console.error('Failed to update article:', error);
      this.showError('更新文章状态失败');
    }
  }

  async addFeed(url: string) {
    try {
      this.setLoading(true);
      const newFeed = await invoke<RssFeed>('add_rss_feed', {
        request: { url }
      });
      
      this.feeds.unshift(newFeed);
      this.renderFeeds();
      await this.loadArticles();
      this.showSuccess('RSS源添加成功');
    } catch (error) {
      console.error('Failed to add feed:', error);
      this.showError('添加RSS源失败: ' + error);
    } finally {
      this.setLoading(false);
    }
  }

  async refreshFeed(feedId: string) {
    try {
      this.setLoading(true);
      const result = await invoke<string>('refresh_rss_feed', { feedId });
      await this.loadArticles(this.currentFeedId || undefined);
      this.showSuccess(result);
    } catch (error) {
      console.error('Failed to refresh feed:', error);
      this.showError('刷新RSS源失败: ' + error);
    } finally {
      this.setLoading(false);
    }
  }

  async deleteFeed(feedId: string) {
    if (!confirm('确定要删除这个RSS源吗？')) return;
    
    try {
      await invoke('delete_rss_feed', { feedId });
      this.feeds = this.feeds.filter(f => f.id !== feedId);
      this.articles = this.articles.filter(a => a.feed_id !== feedId);
      
      if (this.currentFeedId === feedId) {
        this.selectFeed('');
      }
      
      this.renderFeeds();
      this.renderArticles();
      this.showSuccess('RSS源删除成功');
    } catch (error) {
      console.error('Failed to delete feed:', error);
      this.showError('删除RSS源失败');
    }
  }

  updateFeedCounts() {
    const totalUnread = this.articles.filter(a => !a.is_read).length;
    document.getElementById('all-count')!.textContent = totalUnread.toString();
    
    this.feeds.forEach(feed => {
      const unreadCount = this.articles.filter(a => a.feed_id === feed.id && !a.is_read).length;
      const feedElement = document.querySelector(`[data-feed-id="${feed.id}"] .feed-count`);
      if (feedElement) {
        if (unreadCount > 0) {
          feedElement.textContent = unreadCount.toString();
          feedElement.style.display = 'inline';
        } else {
          feedElement.style.display = 'none';
        }
      }
    });
  }

  setLoading(loading: boolean) {
    this.isLoading = loading;
    const loadingElement = document.getElementById('loading')!;
    if (loading) {
      loadingElement.classList.remove('hidden');
    } else {
      loadingElement.classList.add('hidden');
    }
  }

  showSuccess(message: string) {
    // 简单的成功提示，可以后续改进为更好的UI
    console.log('Success:', message);
  }

  showError(message: string) {
    // 简单的错误提示，可以后续改进为更好的UI
    console.error('Error:', message);
    alert(message);
  }

  setupEventListeners() {
    // 添加RSS源按钮
    document.getElementById('add-feed-btn')!.addEventListener('click', () => {
      document.getElementById('add-feed-modal')!.classList.add('active');
    });

    // 关闭模态框
    document.getElementById('close-modal-btn')!.addEventListener('click', () => {
      document.getElementById('add-feed-modal')!.classList.remove('active');
    });

    document.getElementById('cancel-add-btn')!.addEventListener('click', () => {
      document.getElementById('add-feed-modal')!.classList.remove('active');
    });

    // 添加RSS源表单
    document.getElementById('add-feed-form')!.addEventListener('submit', async (e) => {
      e.preventDefault();
      const urlInput = document.getElementById('feed-url')! as HTMLInputElement;
      const url = urlInput.value.trim();
      
      if (url) {
        await this.addFeed(url);
        urlInput.value = '';
        document.getElementById('add-feed-modal')!.classList.remove('active');
      }
    });

    // 刷新按钮
    document.getElementById('refresh-btn')!.addEventListener('click', async () => {
      if (this.currentFeedId) {
        await this.refreshFeed(this.currentFeedId);
      } else {
        // 刷新所有源
        for (const feed of this.feeds) {
          await this.refreshFeed(feed.id);
        }
      }
    });

    // 全部标记为已读
    document.getElementById('mark-all-read-btn')!.addEventListener('click', async () => {
      const unreadArticles = this.articles.filter(a => !a.is_read);
      for (const article of unreadArticles) {
        await this.updateArticleStatus(article.id, { is_read: true });
      }
    });

    // 关闭阅读器
    document.getElementById('close-reader-btn')!.addEventListener('click', () => {
      this.closeReader();
    });

    // 切换收藏状态
    document.getElementById('toggle-star-btn')!.addEventListener('click', async () => {
      if (this.currentArticle) {
        const newStarred = !this.currentArticle.is_starred;
        await this.updateArticleStatus(this.currentArticle.id, { is_starred: newStarred });
        this.currentArticle.is_starred = newStarred;
        
        const starBtn = document.getElementById('toggle-star-btn')!;
        starBtn.textContent = newStarred ? '★' : '☆';
      }
    });

    // 切换已读状态
    document.getElementById('toggle-read-btn')!.addEventListener('click', async () => {
      if (this.currentArticle) {
        const newRead = !this.currentArticle.is_read;
        await this.updateArticleStatus(this.currentArticle.id, { is_read: newRead });
        this.currentArticle.is_read = newRead;
        
        const readBtn = document.getElementById('toggle-read-btn')!;
        readBtn.textContent = newRead ? '已读' : '未读';
      }
    });

    // 打开原文
    document.getElementById('open-original-btn')!.addEventListener('click', () => {
      if (this.currentArticle?.link) {
        window.open(this.currentArticle.link, '_blank');
      }
    });

    // 所有文章按钮
    document.querySelector('.all-feeds')!.addEventListener('click', () => {
      this.selectFeed('');
    });

    // 事件委托处理RSS源操作
    document.addEventListener('click', async (e) => {
      const target = e.target as HTMLElement;
      
      if (target.classList.contains('refresh-feed')) {
        const feedId = target.dataset.feedId!;
        await this.refreshFeed(feedId);
      } else if (target.classList.contains('delete-feed')) {
        const feedId = target.dataset.feedId!;
        await this.deleteFeed(feedId);
      }
    });

    // 点击模态框背景关闭
    document.getElementById('add-feed-modal')!.addEventListener('click', (e) => {
      if (e.target === e.currentTarget) {
        document.getElementById('add-feed-modal')!.classList.remove('active');
      }
    });

    // ESC键关闭模态框和阅读器
    document.addEventListener('keydown', (e) => {
      if (e.key === 'Escape') {
        document.getElementById('add-feed-modal')!.classList.remove('active');
        this.closeReader();
      }
    });
  }
}

// 应用初始化
let app: AppState;

window.addEventListener('DOMContentLoaded', () => {
  app = new AppState();
});

// 导出供调试使用
(window as any).app = app;
