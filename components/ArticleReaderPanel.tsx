'use client';

import React from 'react';
import { Button } from './ui/button';
import { Badge } from './ui/badge';
import { ScrollArea } from './ui/scroll-area';
import { Separator } from './ui/separator';
import { Star, Share2, ExternalLink, Clock, User, Calendar } from 'lucide-react';

interface Article {
  id: string;
  title: string;
  description?: string;
  content?: string;
  link?: string;
  published_at?: string;
  author?: string;
  is_read: boolean;
  is_starred: boolean;
  feed_id: string;
  feedName?: string;
  readTime?: string;
  image?: string;
}

interface ArticleReaderPanelProps {
  selectedArticle: Article | null;
  onUpdateArticle: (articleId: string, updates: Partial<Article>) => void;
  onOpenInBrowser?: (url: string) => void;
}

export default function ArticleReaderPanel({
  selectedArticle,
  onUpdateArticle,
  onOpenInBrowser
}: ArticleReaderPanelProps) {
  if (!selectedArticle) {
    return (
      <div className="flex-1 bg-background flex items-center justify-center">
        <div className="text-center">
          <div className="w-16 h-16 bg-muted rounded-full flex items-center justify-center mx-auto mb-4">
            <ExternalLink className="h-8 w-8 text-muted-foreground" />
          </div>
          <h3 className="text-lg font-medium mb-2">选择一篇文章开始阅读</h3>
          <p className="text-muted-foreground">从左侧列表中选择一篇文章来查看详细内容</p>
        </div>
      </div>
    );
  }

  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    return date.toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
  };

  const handleToggleStar = () => {
    onUpdateArticle(selectedArticle.id, { is_starred: !selectedArticle.is_starred });
  };

  const handleShare = async () => {
    if (navigator.share) {
      try {
        await navigator.share({
          title: selectedArticle.title,
          text: selectedArticle.description,
          url: selectedArticle.link || ''
        });
      } catch (err) {
        console.log('分享失败:', err);
      }
    } else {
      // 复制到剪贴板
      try {
        await navigator.clipboard.writeText(selectedArticle.link || '');
        // 这里可以添加一个 toast 提示
      } catch (err) {
        console.log('复制失败:', err);
      }
    }
  };

  const handleOpenInBrowser = () => {
    if (onOpenInBrowser && selectedArticle.link) {
      onOpenInBrowser(selectedArticle.link);
    } else if (selectedArticle.link) {
      window.open(selectedArticle.link, '_blank');
    }
  };

  return (
    <div className="flex-1 bg-background flex flex-col">
      {/* Header */}
      <div className="border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
        <div className="p-6">
          {/* Article Meta */}
          <div className="flex items-center gap-2 text-sm text-muted-foreground mb-3">
            {selectedArticle.author && (
              <>
                <User className="h-4 w-4" />
                <span>{selectedArticle.author}</span>
                <span>•</span>
              </>
            )}
            <Badge variant="outline">{selectedArticle.feedName || '未知来源'}</Badge>
            <span>•</span>
            <div className="flex items-center gap-1">
              <Calendar className="h-4 w-4" />
              <span>{selectedArticle.published_at ? formatDate(selectedArticle.published_at) : '未知时间'}</span>
            </div>
            {selectedArticle.readTime && (
              <>
                <span>•</span>
                <div className="flex items-center gap-1">
                  <Clock className="h-4 w-4" />
                  <span>{selectedArticle.readTime}</span>
                </div>
              </>
            )}
          </div>

          {/* Title */}
          <h1 className="text-2xl font-bold leading-tight mb-4">
            {selectedArticle.title}
          </h1>

          {/* Description */}
          {selectedArticle.description && (
            <p className="text-lg text-muted-foreground leading-relaxed mb-4">
              {selectedArticle.description}
            </p>
          )}

          {/* Action Buttons */}
          <div className="flex items-center gap-2">
            <Button
              variant={selectedArticle.is_starred ? "default" : "outline"}
              size="sm"
              onClick={handleToggleStar}
            >
              <Star
                className={`h-4 w-4 mr-2 ${
                  selectedArticle.is_starred
                    ? 'fill-current'
                    : ''
                }`}
              />
              {selectedArticle.is_starred ? '已收藏' : '收藏'}
            </Button>
            
            <Button variant="outline" size="sm" onClick={handleShare}>
              <Share2 className="h-4 w-4 mr-2" />
              分享
            </Button>
            
            <Button variant="outline" size="sm" onClick={handleOpenInBrowser}>
              <ExternalLink className="h-4 w-4 mr-2" />
              在浏览器中打开
            </Button>
          </div>
        </div>
      </div>

      {/* Content */}
      <ScrollArea className="flex-1">
        <div className="p-6">
          {/* Article Image */}
          {selectedArticle.image && (
            <div className="mb-6">
              <img
                src={selectedArticle.image}
                alt={selectedArticle.title}
                className="w-full h-auto rounded-lg shadow-sm"
                onError={(e) => {
                  const target = e.target as HTMLImageElement;
                  target.style.display = 'none';
                }}
              />
            </div>
          )}

          {/* Article Content */}
          <div className="prose prose-gray dark:prose-invert max-w-none">
            {selectedArticle.content ? (
              <div
                dangerouslySetInnerHTML={{ __html: selectedArticle.content }}
                className="leading-relaxed"
              />
            ) : (
              <div className="text-center py-8">
                <div className="w-16 h-16 bg-muted rounded-full flex items-center justify-center mx-auto mb-4">
                  <ExternalLink className="h-8 w-8 text-muted-foreground" />
                </div>
                <h3 className="text-lg font-medium mb-2">正在加载内容...</h3>
                <p className="text-muted-foreground mb-4">
                  系统正在获取完整文章内容，请稍候
                </p>
                <Button onClick={handleOpenInBrowser}>
                  <ExternalLink className="h-4 w-4 mr-2" />
                  在浏览器中打开
                </Button>
              </div>
            )}
          </div>

          {/* Footer */}
          <Separator className="my-8" />
          <div className="text-center text-sm text-muted-foreground">
            <p>来源: <a href={selectedArticle.link} target="_blank" rel="noopener noreferrer" className="text-primary hover:underline">{selectedArticle.feedName || '未知来源'}</a></p>
            <p className="mt-1">发布时间: {formatDate(selectedArticle.published_at || '')}</p>
          </div>
        </div>
      </ScrollArea>
    </div>
  );
}