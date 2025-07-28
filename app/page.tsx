"use client";

import { useState, useEffect } from "react";
import RSSFeedPanel from "@/components/RSSFeedPanel";
import ArticleListPanel from "@/components/ArticleListPanel";
import ArticleReaderPanel from "@/components/ArticleReaderPanel";
import { ApiService, RssFeed, RssArticle } from "@/services/api";

// 定义Tauri窗口接口
interface TauriWindow extends Window {
  __TAURI__?: unknown;
}

export interface ExtendedArticle extends RssArticle {
  feedName?: string;
  readTime?: string;
  image?: string;
  summary?: string;
}

export default function HomePage() {
  const [selectedFeed, setSelectedFeed] = useState<string | null>(null);
  const [selectedArticle, setSelectedArticle] =
    useState<ExtendedArticle | null>(null);
  const [searchQuery, setSearchQuery] = useState("");
  const [feeds, setFeeds] = useState<RssFeed[]>([]);
  const [articles, setArticles] = useState<ExtendedArticle[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // 添加Esc键退出阅读模式的功能
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape" && selectedArticle) {
        setSelectedArticle(null);
      }
    };

    document.addEventListener("keydown", handleKeyDown);
    return () => {
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, [selectedArticle]);

  // Load data from API
  const loadData = async () => {
    try {
      setLoading(true);
      setError(null);
      // Check if we're in Tauri environment
      if (typeof window !== "undefined" && (window as TauriWindow).__TAURI__) {
        // Initialize database tables first
        await ApiService.createTables();
        const feedsData = await ApiService.getRssFeeds();
        const articlesData = await ApiService.getArticles();
        setFeeds(feedsData);
        setArticles(
          articlesData.map((article) => ({
            ...article,
            feedName:
              feedsData.find((f) => f.id === article.feed_id)?.title ||
              "Unknown Feed",
            readTime: article.read_time || undefined,
            summary:
              article.description ||
              article.content?.substring(0, 200) + "..." ||
              "No summary available",
          })),
        );
      } else {
        // Running in browser development mode, use empty data
        console.log("Running in browser mode, no Tauri API available");
        setError(
          "Running in browser mode - RSS feeds are only available in the desktop app",
        );
        setFeeds([]);
        setArticles([]);
      }
    } catch (err) {
      console.error("Failed to load data:", err);
      setError("Failed to load data: " + (err as Error).message);
      // Fallback to empty data on error
      setFeeds([]);
      setArticles([]);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadData();
  }, []);

  // Add RSS feed function
  const handleAddFeed = async (url: string) => {
    try {
      setError(null);
      if (typeof window !== "undefined" && (window as TauriWindow).__TAURI__) {
        const newFeed = await ApiService.addRssFeed({ url });
        setFeeds((prev) => [...prev, newFeed]);
        // Refresh articles after adding feed
        const articlesData = await ApiService.getArticles();
        setArticles(
          articlesData.map((article) => ({
            ...article,
            feedName:
              [...feeds, newFeed].find((f) => f.id === article.feed_id)
                ?.title || "Unknown Feed",
            readTime: article.read_time || undefined,
            summary:
              article.description ||
              article.content?.substring(0, 200) + "..." ||
              "No summary available",
          })),
        );
        return true;
      } else {
        console.log("Cannot add feed in browser mode");
        setError("Adding feeds is only available in the desktop app");
        return false;
      }
    } catch (err) {
      console.error("Failed to add feed:", err);
      setError("Failed to add feed: " + (err as Error).message);
      return false;
    }
  };

  // Update article status
  const handleUpdateArticle = async (
    articleId: string,
    updates: { is_read?: boolean; is_starred?: boolean },
  ) => {
    try {
      if (typeof window !== "undefined" && (window as TauriWindow).__TAURI__) {
        await ApiService.updateArticle({ id: articleId, ...updates });
      }
      // Update local state regardless of API call
      setArticles((prev) =>
        prev.map((article) =>
          article.id === articleId ? { ...article, ...updates } : article,
        ),
      );
      if (selectedArticle?.id === articleId) {
        setSelectedArticle((prev) => (prev ? { ...prev, ...updates } : null));
      }
    } catch (err) {
      console.error("Failed to update article:", err);
    }
  };

  // Refresh feed
  const handleRefreshFeed = async (feedId: string) => {
    try {
      await ApiService.refreshRssFeed(feedId);
      // Reload articles after refresh
      const articlesData = await ApiService.getArticles();
      const enrichedArticles = articlesData.map((article) => ({
        ...article,
        feedName:
          feeds.find((f) => f.id === article.feed_id)?.title || "Unknown",
        readTime: article.read_time || undefined,
        summary: article.content?.substring(0, 200) + "..." || "",
      }));
      setArticles(enrichedArticles);
    } catch (error) {
      console.error("Failed to refresh feed:", error);
      setError("Failed to refresh feed");
    }
  };

  // Delete feed
  const handleDeleteFeed = async (feedId: string, feedTitle: string) => {
    if (confirm(`Are you sure you want to delete "${feedTitle}"?`)) {
      try {
        await ApiService.deleteRssFeed(feedId);
        // Reload feeds and articles
        const feedsData = await ApiService.getRssFeeds();
        setFeeds(feedsData);
        const articlesData = await ApiService.getArticles();
        const enrichedArticles = articlesData.map((article) => ({
          ...article,
          feedName:
            feedsData.find((f) => f.id === article.feed_id)?.title || "Unknown",
          readTime: article.read_time || undefined,
          summary: article.content?.substring(0, 200) + "..." || "",
        }));
        setArticles(enrichedArticles);
        // Reset selected feed if it was deleted
        if (selectedFeed === feedId) {
          setSelectedFeed(null);
        }
      } catch (error) {
        console.error("Failed to delete feed:", error);
        setError("Failed to delete feed");
      }
    }
  };

  const filteredArticles = articles.filter((article) => {
    if (selectedFeed && article.feed_id !== selectedFeed) return false;
    if (
      searchQuery &&
      !article.title.toLowerCase().includes(searchQuery.toLowerCase())
    )
      return false;
    return true;
  });

  return (
    <div className="flex h-screen bg-gradient-to-br from-slate-50 via-white to-blue-50 dark:from-slate-950 dark:via-slate-900 dark:to-blue-950 text-foreground">
      {/* RSS Feeds Panel */}
      <div
        className={`transition-all duration-300 ease-in-out ${
          selectedArticle ? "hidden" : "w-[40%] min-w-[300px]"
        } border-r border-gray-200 dark:border-gray-700`}
      >
        <RSSFeedPanel
          feeds={feeds.map(feed => ({
            ...feed,
            unread_count: articles.filter(article => 
              article.feed_id === feed.id && !article.is_read
            ).length
          }))}
          articles={articles}
          selectedFeed={selectedFeed}
          searchQuery={searchQuery}
          loading={loading}
          error={error}
          onFeedSelect={setSelectedFeed}
          onSearchChange={setSearchQuery}
          onAddFeed={handleAddFeed}
          onRefreshData={loadData}
          onRefreshFeed={handleRefreshFeed}
          onDeleteFeed={handleDeleteFeed}
          isCollapsed={!!selectedArticle}
        />
      </div>

      {/* Article List Panel */}
      <div
        className={`transition-all duration-300 ease-in-out ${
          selectedArticle ? "hidden" : "w-[60%] min-w-[400px]"
        } border-r border-gray-200 dark:border-gray-700`}
      >
        <ArticleListPanel
          articles={filteredArticles}
          feeds={feeds}
          selectedFeed={selectedFeed}
          selectedArticle={selectedArticle}
          loading={loading}
          error={error}
          isCollapsed={!!selectedArticle}
          onArticleSelect={async (article) => {
            setSelectedArticle(article);
            // Mark article as read if it's not already read
            if (!article.is_read) {
              handleUpdateArticle(article.id, { is_read: true });
            }

            // Get full article content if not already loaded
            if (
              typeof window !== "undefined" &&
              (window as TauriWindow).__TAURI__
            ) {
              try {
                const fullArticle = await ApiService.getArticleContent(
                  article.id,
                );
                const enrichedArticle = {
                  ...fullArticle,
                  feedName:
                    feeds.find((f) => f.id === fullArticle.feed_id)?.title ||
                    "Unknown Feed",
                  readTime: fullArticle.read_time || undefined,
                  summary:
                    fullArticle.description ||
                    fullArticle.content?.substring(0, 200) + "..." ||
                    "No summary available",
                };
                setSelectedArticle(enrichedArticle);
              } catch (err) {
                console.error("Failed to load article content:", err);
              }
            }
          }}
        />
      </div>

      {/* Article Reader Panel */}
      <div
        className={`transition-all duration-300 ease-in-out ${
          selectedArticle ? "w-full" : "hidden"
        }`}
      >
        <ArticleReaderPanel
          selectedArticle={selectedArticle}
          onUpdateArticle={handleUpdateArticle}
          onClose={() => setSelectedArticle(null)}
        />
      </div>
    </div>
  );
}
