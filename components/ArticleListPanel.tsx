'use client';

import React, { useState } from 'react';
import { Button } from './ui/button';
import { Input } from './ui/input';
import { ScrollArea } from './ui/scroll-area';
import { Badge } from './ui/badge';
import { Tabs, TabsContent, TabsList, TabsTrigger } from './ui/tabs';
import { Search, Star, Clock, ExternalLink } from 'lucide-react';

interface Article {
  id: string;
  title: string;
  description?: string;
  link?: string;
  published_at?: string;
  is_read: boolean;
  is_starred: boolean;
  feed_id: string;
  feedName?: string;
  readTime?: string;
}

interface ArticleListPanelProps {
  articles: Article[];
  feeds: any[];
  selectedFeed: string | null;
  selectedArticle: Article | null;
  loading: boolean;
  error: string | null;
  onArticleSelect: (article: Article) => void;
}

export default function ArticleListPanel({
  articles,
  feeds,
  selectedFeed,
  selectedArticle,
  loading,
  error,
  onArticleSelect
}: ArticleListPanelProps) {
  const [searchTerm, setSearchTerm] = useState('');
  const [filter, setFilter] = useState<'all' | 'unread' | 'starred'>('all');

  const filteredArticles = articles.filter(article => {
    const matchesSearch = article.title.toLowerCase().includes(searchTerm.toLowerCase()) ||
                         article.description?.toLowerCase().includes(searchTerm.toLowerCase()) ||
                         (article.feedName && article.feedName.toLowerCase().includes(searchTerm.toLowerCase()));
    
    const matchesFilter = 
      filter === 'all' ||
      (filter === 'unread' && !article.is_read) ||
      (filter === 'starred' && article.is_starred);
    
    return matchesSearch && matchesFilter;
  });

  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    const now = new Date();
    const diffInHours = Math.floor((now.getTime() - date.getTime()) / (1000 * 60 * 60));
    
    if (diffInHours < 1) {
      return '刚刚';
    } else if (diffInHours < 24) {
      return `${diffInHours}小时前`;
    } else if (diffInHours < 48) {
      return '昨天';
    } else {
      return date.toLocaleDateString('zh-CN', {
        month: 'short',
        day: 'numeric'
      });
    }
  };



  if (loading) {
    return (
      <div className="w-96 border-r bg-background flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary mx-auto mb-2"></div>
          <p className="text-sm text-muted-foreground">加载中...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="w-96 border-r bg-background flex flex-col">
      {/* Header */}
      <div className="p-4 border-b">
        <div className="flex items-center justify-between mb-3">
          <h2 className="text-lg font-semibold">文章列表</h2>
          <Badge variant="outline">
            {filteredArticles.length}
          </Badge>
        </div>
        
        {/* Search */}
        <div className="relative mb-3">
          <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-muted-foreground" />
          <Input
            placeholder="搜索文章..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="pl-9"
          />
        </div>

        {/* Filter Tabs */}
        <Tabs value={filter} onValueChange={(value) => setFilter(value as any)}>
          <TabsList className="grid w-full grid-cols-3">
            <TabsTrigger value="all">全部</TabsTrigger>
            <TabsTrigger value="unread">未读</TabsTrigger>
            <TabsTrigger value="starred">已收藏</TabsTrigger>
          </TabsList>
        </Tabs>
      </div>

      {/* Article List */}
      <ScrollArea className="flex-1">
        <div className="p-2">
          {filteredArticles.length === 0 ? (
            <div className="text-center py-8">
              <p className="text-muted-foreground">没有找到文章</p>
            </div>
          ) : (
            filteredArticles.map((article) => (
              <div
                key={article.id}
                className={`p-4 rounded-lg cursor-pointer transition-colors mb-2 border ${
                  selectedArticle?.id === article.id
                    ? 'bg-primary/10 border-primary'
                    : 'hover:bg-muted border-transparent'
                } ${
                  !article.is_read ? 'bg-blue-50/50 dark:bg-blue-950/20' : ''
                }`}
                onClick={() => onArticleSelect(article)}
              >
                <div className="flex items-start justify-between mb-2">
                  <div className="flex-1 min-w-0">
                    <h3 className={`font-medium line-clamp-2 ${
                      !article.is_read ? 'font-semibold' : ''
                    }`}>
                      {article.title}
                    </h3>
                  </div>
                  <Button
                    size="sm"
                    variant="ghost"
                    onClick={(e) => {
                      e.stopPropagation();
                      // TODO: 实现星标功能
                    }}
                    className="ml-2 flex-shrink-0"
                  >
                    <Star
                      className={`h-4 w-4 ${
                        article.is_starred
                          ? 'fill-yellow-400 text-yellow-400'
                          : 'text-muted-foreground'
                      }`}
                    />
                  </Button>
                </div>

                {article.description && (
                  <p className="text-sm text-muted-foreground line-clamp-2 mb-2">
                    {article.description}
                  </p>
                )}

                <div className="flex items-center justify-between text-xs text-muted-foreground">
                  <div className="flex items-center gap-2">
                    <span className="font-medium">{article.feedName || '未知来源'}</span>
                    <span>•</span>
                    <div className="flex items-center gap-1">
                      <Clock className="h-3 w-3" />
                      <span>{article.published_at ? formatDate(article.published_at) : '未知时间'}</span>
                    </div>
                    {article.readTime && (
                      <>
                        <span>•</span>
                        <span>{article.readTime}</span>
                      </>
                    )}
                  </div>
                  
                  {!article.is_read && (
                    <div className="w-2 h-2 bg-blue-500 rounded-full"></div>
                  )}
                </div>

                <div className="flex items-center gap-2 mt-2">
                  {article.is_starred && (
                    <Badge variant="secondary" className="text-xs">
                      已收藏
                    </Badge>
                  )}
                  {!article.is_read && (
                    <Badge variant="default" className="text-xs">
                      未读
                    </Badge>
                  )}
                </div>
              </div>
            ))
          )}
        </div>
      </ScrollArea>
    </div>
  );
}