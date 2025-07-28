"use client";

import React from "react";
import { Globe, X } from "lucide-react";
import { Button } from "./ui/button";

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
  onUpdateArticle?: (articleId: string, updates: Partial<Article>) => void;
  onOpenInBrowser?: (url: string) => void;
  onClose?: () => void;
}

export default function ArticleReaderPanel({
  selectedArticle,
  onClose,
}: ArticleReaderPanelProps) {
  if (!selectedArticle) {
    return (
      <div className="h-full bg-background flex items-center justify-center">
        <div className="text-center">
          <div className="w-16 h-16 bg-muted rounded-full flex items-center justify-center mx-auto mb-4">
            <Globe className="h-8 w-8 text-muted-foreground" />
          </div>
          <h3 className="text-lg font-medium mb-2">选择一篇文章开始阅读</h3>
          <p className="text-muted-foreground">
            从左侧列表中选择一篇文章来查看详细内容
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className="h-full bg-background flex flex-col relative">
      {/* 关闭按钮 */}
      {onClose && (
        <Button
          variant="outline"
          size="sm"
          className="absolute top-4 right-4 z-10 bg-background/80 backdrop-blur-sm hover:bg-background"
          onClick={onClose}
        >
          <X className="h-4 w-4" />
        </Button>
      )}

      {/* 直接显示嵌入的网页 */}
      <div className="flex-1 relative">
        {selectedArticle.link ? (
          /* iframe嵌入原始网页 */
          <iframe
            src={selectedArticle.link}
            className="w-full h-full border-0"
            title={selectedArticle.title}
            sandbox="allow-scripts allow-same-origin allow-forms allow-popups allow-popups-to-escape-sandbox"
            onError={() => {
              console.log("iframe加载失败");
            }}
          />
        ) : (
          <div className="flex items-center justify-center h-full">
            <div className="text-center">
              <div className="w-16 h-16 bg-muted rounded-full flex items-center justify-center mx-auto mb-4">
                <Globe className="h-8 w-8 text-muted-foreground" />
              </div>
              <h3 className="text-lg font-medium mb-2">无法加载网页</h3>
              <p className="text-muted-foreground">该文章没有提供原始链接</p>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
