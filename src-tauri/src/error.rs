use thiserror::Error;

/// 应用统一错误类型
#[derive(Error, Debug)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    Database(#[from] sqlx::Error),

    #[error("数据库迁移错误: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("HTTP请求错误: {0}")]
    Http(#[from] reqwest::Error),

    #[error("RSS解析错误: {0}")]
    RssParse(#[from] feed_rs::parser::ParseFeedError),

    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("序列化错误: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("UUID解析错误: {0}")]
    Uuid(#[from] uuid::Error),

    #[error("URL解析错误: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Tauri错误: {0}")]
    Tauri(#[from] tauri::Error),

    #[error("RSS源未找到: {id}")]
    FeedNotFound { id: String },

    #[error("文章未找到: {id}")]
    ArticleNotFound { id: String },

    #[error("无效的RSS URL: {url}")]
    InvalidRssUrl { url: String },

    #[error("RSS源已存在: {url}")]
    FeedAlreadyExists { url: String },

    #[error("配置错误: {message}")]
    Config { message: String },

    #[error("验证错误: {message}")]
    Validation { message: String },

    #[error("内部错误: {message}")]
    Internal { message: String },
}

/// 应用统一结果类型
pub type AppResult<T> = Result<T, AppError>;

/// 为Tauri命令实现序列化
impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// 便捷的错误构造函数
impl AppError {
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
        }
    }

    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }

    pub fn feed_not_found(id: impl Into<String>) -> Self {
        Self::FeedNotFound { id: id.into() }
    }

    pub fn article_not_found(id: impl Into<String>) -> Self {
        Self::ArticleNotFound { id: id.into() }
    }

    pub fn invalid_rss_url(url: impl Into<String>) -> Self {
        Self::InvalidRssUrl { url: url.into() }
    }

    pub fn feed_already_exists(url: impl Into<String>) -> Self {
        Self::FeedAlreadyExists { url: url.into() }
    }
}
