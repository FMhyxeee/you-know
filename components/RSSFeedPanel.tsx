'use client';

import React, { useState } from 'react';
import { Button } from './ui/button';
import { Input } from './ui/input';
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogTrigger } from './ui/dialog';
import { Label } from './ui/label';
import { ScrollArea } from './ui/scroll-area';
import { Badge } from './ui/badge';
import { Plus, Search, RefreshCw, Trash2 } from 'lucide-react';

interface Feed {
  id: string;
  title: string;
  url: string;
  description?: string;
  unread_count: number;
}

interface RSSFeedPanelProps {
  feeds: any[];
  articles: any[];
  selectedFeed: string | null;
  searchQuery: string;
  loading: boolean;
  error: string | null;
  onFeedSelect: (feedId: string | null) => void;
  onSearchChange: (query: string) => void;
  onAddFeed: (url: string) => Promise<boolean>;
  onRefreshData: () => void;
  onRefreshFeed: (feedId: string) => void;
  onDeleteFeed: (feedId: string, feedTitle: string) => void;
}

export default function RSSFeedPanel({
  feeds,
  articles,
  selectedFeed,
  searchQuery,
  loading,
  error,
  onFeedSelect,
  onSearchChange,
  onAddFeed,
  onRefreshData,
  onRefreshFeed,
  onDeleteFeed
}: RSSFeedPanelProps) {
  const [isAddDialogOpen, setIsAddDialogOpen] = useState(false);
  const [newFeedUrl, setNewFeedUrl] = useState('');
  const [statistics, setStatistics] = useState<any>(null);

  // 计算每个feed的未读数量
  const feedsWithUnreadCount = feeds.map(feed => ({
    ...feed,
    unread_count: articles.filter(article => article.feed_id === feed.id && !article.is_read).length
  }));

  const filteredFeeds = feedsWithUnreadCount.filter(feed =>
    feed.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
    feed.url.toLowerCase().includes(searchQuery.toLowerCase())
  );

  const handleAddFeed = async () => {
    if (newFeedUrl.trim()) {
      const success = await onAddFeed(newFeedUrl.trim());
      if (success) {
        setNewFeedUrl('');
        setIsAddDialogOpen(false);
      }
    }
  };

  // 计算总的未读数量
  const totalUnreadCount = articles.filter(article => !article.is_read).length;

  return (
    <div className="w-80 border-r bg-muted/50 flex flex-col">
      {/* Header */}
      <div className="p-4 border-b">
        <div className="flex items-center justify-between mb-3">
          <h2 className="text-lg font-semibold">RSS 订阅源</h2>
          <Dialog open={isAddDialogOpen} onOpenChange={setIsAddDialogOpen}>
            <DialogTrigger asChild>
              <Button size="sm" variant="outline">
                <Plus className="h-4 w-4" />
              </Button>
            </DialogTrigger>
            <DialogContent>
              <DialogHeader>
                <DialogTitle>添加 RSS 订阅源</DialogTitle>
              </DialogHeader>
              <div className="space-y-4">
                <div>
                  <Label htmlFor="feed-url">RSS URL *</Label>
                  <Input
                    id="feed-url"
                    placeholder="https://example.com/rss"
                    value={newFeedUrl}
                    onChange={(e) => setNewFeedUrl(e.target.value)}
                  />
                </div>
                <Button onClick={handleAddFeed} className="w-full">
                  添加订阅源
                </Button>
              </div>
            </DialogContent>
          </Dialog>
        </div>
        
        {/* Search */}
        <div className="relative">
          <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-muted-foreground" />
          <Input
            placeholder="搜索订阅源..."
            value={searchQuery}
            onChange={(e) => onSearchChange(e.target.value)}
            className="pl-9"
          />
        </div>
      </div>

      {/* Feed List */}
      <ScrollArea className="flex-1">
        <div className="p-2">
          {/* All Articles Option */}
          <div
            className={`p-3 rounded-lg cursor-pointer transition-colors mb-2 ${
              selectedFeed === null
                ? 'bg-primary text-primary-foreground'
                : 'hover:bg-muted'
            }`}
            onClick={() => onFeedSelect(null)}
          >
            <div className="flex items-center justify-between">
              <span className="font-medium">所有文章</span>
              <Badge variant="secondary">
                {totalUnreadCount}
              </Badge>
            </div>
          </div>

          {/* Individual Feeds */}
          {filteredFeeds.map((feed) => (
            <div
              key={feed.id}
              className={`p-3 rounded-lg cursor-pointer transition-colors mb-2 group ${
                selectedFeed === feed.id
                  ? 'bg-primary text-primary-foreground'
                  : 'hover:bg-muted'
              }`}
              onClick={() => onFeedSelect(feed.id)}
            >
              <div className="flex items-start justify-between">
                <div className="flex-1 min-w-0">
                  <div className="flex items-center justify-between mb-1">
                    <span className="font-medium truncate">{feed.title}</span>
                    {feed.unread_count > 0 && (
                      <Badge variant="secondary" className="ml-2">
                        {feed.unread_count}
                      </Badge>
                    )}
                  </div>
                  {feed.description && (
                    <p className="text-sm text-muted-foreground truncate">
                      {feed.description}
                    </p>
                  )}
                </div>
              </div>
              
              {/* Action Buttons */}
              <div className="flex items-center gap-1 mt-2 opacity-0 group-hover:opacity-100 transition-opacity">
                <Button
                  size="sm"
                  variant="ghost"
                  onClick={(e) => {
                    e.stopPropagation();
                    onRefreshFeed(feed.id);
                  }}
                  disabled={loading}
                >
                  <RefreshCw className={`h-3 w-3 ${loading ? 'animate-spin' : ''}`} />
                </Button>
                <Button
                  size="sm"
                  variant="ghost"
                  onClick={(e) => {
                    e.stopPropagation();
                    onDeleteFeed(feed.id, feed.title);
                  }}
                >
                  <Trash2 className="h-3 w-3" />
                </Button>
              </div>
            </div>
          ))}
        </div>
      </ScrollArea>
    </div>
  );
}