# 系统模式

## 架构模式

### 整体架构
```
┌─────────────────────────────────────────────────────────────┐
│                    用户界面层 (React)                        │
├─────────────────────────────────────────────────────────────┤
│                    状态管理层 (Zustand)                      │
├─────────────────────────────────────────────────────────────┤
│                    服务层 (Tauri Commands)                   │
├─────────────────────────────────────────────────────────────┤
│                    业务逻辑层 (Rust)                         │
├─────────────────────────────────────────────────────────────┤
│                    数据访问层 (SQLite)                       │
├─────────────────────────────────────────────────────────────┤
│                    外部服务层 (LLM APIs)                     │
└─────────────────────────────────────────────────────────────┘
```

### 模块化设计
- **组件化**: 高内聚、低耦合的组件设计
- **服务化**: 独立的服务模块
- **插件化**: 可扩展的插件系统
- **配置化**: 配置驱动的功能

## 设计模式

### 前端设计模式

#### 1. 组件模式
```typescript
// 容器组件 - 逻辑处理
const ChatContainer: React.FC = () => {
  const { messages, sendMessage } = useChat();
  
  return (
    <ChatView 
      messages={messages}
      onSendMessage={sendMessage}
    />
  );
};

// 展示组件 - 纯UI
const ChatView: React.FC<ChatViewProps> = ({ messages, onSendMessage }) => {
  return (
    <div className="chat-view">
      {/* UI实现 */}
    </div>
  );
};
```

#### 2. 状态管理模式
```typescript
// Zustand Store
interface ChatState {
  messages: Message[];
  isLoading: boolean;
  currentProvider: LLMProvider;
  
  // Actions
  addMessage: (message: Message) => void;
  setLoading: (loading: boolean) => void;
  setProvider: (provider: LLMProvider) => void;
}

const useChatStore = create<ChatState>((set) => ({
  messages: [],
  isLoading: false,
  currentProvider: 'openai',
  
  addMessage: (message) => set(state => ({
    messages: [...state.messages, message]
  })),
  
  setLoading: (loading) => set({ isLoading: loading }),
  setProvider: (provider) => set({ currentProvider: provider })
}));
```

#### 3. 服务层模式
```typescript
// 服务接口
interface LLMService {
  sendMessage(message: string, config: LLMConfig): Promise<string>;
  streamMessage(message: string, config: LLMConfig): AsyncGenerator<string>;
}

// 具体实现
class OpenAIService implements LLMService {
  async sendMessage(message: string, config: LLMConfig): Promise<string> {
    // OpenAI API实现
  }
  
  async* streamMessage(message: string, config: LLMConfig): AsyncGenerator<string> {
    // 流式响应实现
  }
}
```

### 后端设计模式

#### 1. 命令模式
```rust
// Tauri命令接口
#[tauri::command]
async fn send_message(
    message: String,
    provider: String,
    config: LLMConfig,
) -> Result<String, String> {
    let service = LLMServiceFactory::create(&provider)?;
    service.send_message(message, config).await
        .map_err(|e| e.to_string())
}
```

#### 2. 工厂模式
```rust
// LLM服务工厂
pub struct LLMServiceFactory;

impl LLMServiceFactory {
    pub fn create(provider: &str) -> Result<Box<dyn LLMService>, Error> {
        match provider {
            "openai" => Ok(Box::new(OpenAIService::new())),
            "anthropic" => Ok(Box::new(AnthropicService::new())),
            "google" => Ok(Box::new(GoogleService::new())),
            _ => Err(Error::UnsupportedProvider(provider.to_string())),
        }
    }
}
```

#### 3. 策略模式
```rust
// 消息处理策略
pub trait MessageProcessor {
    fn process(&self, message: &str) -> Result<String, Error>;
}

pub struct OpenAIProcessor;
impl MessageProcessor for OpenAIProcessor {
    fn process(&self, message: &str) -> Result<String, Error> {
        // OpenAI特定处理
    }
}

pub struct AnthropicProcessor;
impl MessageProcessor for AnthropicProcessor {
    fn process(&self, message: &str) -> Result<String, Error> {
        // Anthropic特定处理
    }
}
```

## 数据流模式

### 单向数据流
```
User Action → Component → Store → Tauri Command → Rust Service → External API
     ↓                                                                    ↓
UI Update ← Component ← Store ← Tauri Event ← Rust Service ← API Response
```

### 事件驱动
```typescript
// 事件总线
class EventBus {
  private listeners: Map<string, Function[]> = new Map();
  
  subscribe(event: string, callback: Function) {
    if (!this.listeners.has(event)) {
      this.listeners.set(event, []);
    }
    this.listeners.get(event)!.push(callback);
  }
  
  emit(event: string, data: any) {
    const callbacks = this.listeners.get(event) || [];
    callbacks.forEach(callback => callback(data));
  }
}
```

## 错误处理模式

### 统一错误处理
```typescript
// 错误类型定义
interface AppError {
  code: string;
  message: string;
  details?: any;
}

// 错误处理Hook
const useErrorHandler = () => {
  const handleError = (error: AppError) => {
    // 记录错误
    console.error('[Error]', error);
    
    // 用户友好提示
    toast.error(error.message);
    
    // 错误上报
    // errorReporter.report(error);
  };
  
  return { handleError };
};
```

### Rust错误处理
```rust
// 自定义错误类型
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    
    #[error("Configuration error: {message}")]
    Config { message: String },
    
    #[error("LLM provider error: {provider} - {message}")]
    LLMProvider { provider: String, message: String },
}

// 结果类型
pub type AppResult<T> = Result<T, AppError>;
```

## 配置管理模式

### 配置层次
```typescript
// 配置接口
interface AppConfig {
  llm: LLMConfig;
  ui: UIConfig;
  mcp: MCPConfig;
  storage: StorageConfig;
}

interface LLMConfig {
  providers: LLMProviderConfig[];
  defaultProvider: string;
  timeout: number;
}

interface UIConfig {
  theme: 'light' | 'dark' | 'system';
  language: string;
  fontSize: number;
}
```

### 配置加载
```rust
// 配置管理器
pub struct ConfigManager {
    store: Arc<Mutex<Store>>,
}

impl ConfigManager {
    pub async fn load_config(&self) -> AppResult<AppConfig> {
        let store = self.store.lock().await;
        
        // 加载配置文件
        let config = store.get("app_config").await?;
        
        // 解析配置
        let config: AppConfig = serde_json::from_value(config)?;
        
        Ok(config)
    }
    
    pub async fn save_config(&self, config: &AppConfig) -> AppResult<()> {
        let store = self.store.lock().await;
        
        // 序列化配置
        let value = serde_json::to_value(config)?;
        
        // 保存配置
        store.set("app_config", value).await?;
        
        Ok(())
    }
}
```

## 性能优化模式

### 前端性能优化
```typescript
// 虚拟滚动
const VirtualMessageList: React.FC<{ messages: Message[] }> = ({ messages }) => {
  const [visibleRange, setVisibleRange] = useState({ start: 0, end: 50 });
  
  const visibleMessages = messages.slice(visibleRange.start, visibleRange.end);
  
  return (
    <div className="virtual-list">
      {visibleMessages.map(message => (
        <MessageItem key={message.id} message={message} />
      ))}
    </div>
  );
};

// 防抖处理
const useDebounce = (value: string, delay: number) => {
  const [debouncedValue, setDebouncedValue] = useState(value);
  
  useEffect(() => {
    const handler = setTimeout(() => {
      setDebouncedValue(value);
    }, delay);
    
    return () => clearTimeout(handler);
  }, [value, delay]);
  
  return debouncedValue;
};
```

### 后端性能优化
```rust
// 连接池
pub struct ConnectionPool {
    pool: Arc<Mutex<Vec<HttpClient>>>,
    max_connections: usize,
}

impl ConnectionPool {
    pub async fn get_connection(&self) -> AppResult<HttpClient> {
        let mut pool = self.pool.lock().await;
        
        if let Some(client) = pool.pop() {
            Ok(client)
        } else {
            Ok(HttpClient::new())
        }
    }
    
    pub async fn return_connection(&self, client: HttpClient) {
        let mut pool = self.pool.lock().await;
        
        if pool.len() < self.max_connections {
            pool.push(client);
        }
    }
}
```

## 测试模式

### 前端测试
```typescript
// 组件测试
describe('ChatMessage', () => {
  it('should render message correctly', () => {
    const message = {
      id: '1',
      content: 'Hello world',
      role: 'user'
    };
    
    render(<ChatMessage message={message} />);
    
    expect(screen.getByText('Hello world')).toBeInTheDocument();
  });
});

// Hook测试
describe('useChat', () => {
  it('should send message correctly', async () => {
    const { result } = renderHook(() => useChat());
    
    await act(async () => {
      await result.current.sendMessage('Hello');
    });
    
    expect(result.current.messages).toHaveLength(1);
  });
});
```

### 后端测试
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_openai_service() {
        let service = OpenAIService::new();
        let result = service.send_message("Hello".to_string(), LLMConfig::default()).await;
        
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_config_manager() {
        let manager = ConfigManager::new();
        let config = AppConfig::default();
        
        manager.save_config(&config).await.unwrap();
        let loaded = manager.load_config().await.unwrap();
        
        assert_eq!(config, loaded);
    }
}
```

## 部署模式

### 构建管道
```yaml
# GitHub Actions
name: Build and Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    
    steps:
    - uses: actions/checkout@v3
    - name: Setup Node.js
      uses: actions/setup-node@v3
      with:
        node-version: '18'
    
    - name: Install dependencies
      run: pnpm install
    
    - name: Build frontend
      run: pnpm build
    
    - name: Build Tauri app
      run: pnpm tauri build
    
    - name: Upload artifacts
      uses: actions/upload-artifact@v3
      with:
        name: app-${{ matrix.os }}
        path: src-tauri/target/release/bundle/
```

### 版本管理
```json
{
  "version": "0.1.0",
  "tauri": {
    "bundle": {
      "identifier": "com.example.ai-chat",
      "version": "0.1.0"
    }
  }
}
``` 

## 数据模型设计

### 数据库表结构 (SQLx + SQLite)

#### 1. 用户会话表 (conversations)
```sql
CREATE TABLE conversations (
    id TEXT PRIMARY KEY,  -- UUID
    title TEXT NOT NULL,
    model_provider TEXT NOT NULL,  -- 'openai', 'anthropic', 'google'
    model_name TEXT NOT NULL,      -- 'gpt-4', 'claude-3', 'gemini-pro'
    system_prompt TEXT,
    temperature REAL DEFAULT 0.7,
    max_tokens INTEGER DEFAULT 2048,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

#### 2. 消息表 (messages)
```sql
CREATE TABLE messages (
    id TEXT PRIMARY KEY,  -- UUID
    conversation_id TEXT NOT NULL,
    role TEXT NOT NULL,   -- 'user', 'assistant', 'system'
    content TEXT NOT NULL,
    token_count INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (conversation_id) REFERENCES conversations(id) ON DELETE CASCADE
);
```

#### 3. 提供商配置表 (provider_configs)
```sql
CREATE TABLE provider_configs (
    id TEXT PRIMARY KEY,  -- UUID
    provider_name TEXT NOT NULL UNIQUE,  -- 'openai', 'anthropic'
    api_key TEXT NOT NULL,               -- 加密存储
    base_url TEXT,                       -- 自定义API地址
    is_enabled BOOLEAN DEFAULT TRUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

#### 4. MCP服务器配置表 (mcp_servers)
```sql
CREATE TABLE mcp_servers (
    id TEXT PRIMARY KEY,  -- UUID
    name TEXT NOT NULL,
    description TEXT,
    endpoint TEXT NOT NULL,
    is_enabled BOOLEAN DEFAULT TRUE,
    config_json TEXT,                    -- JSON格式的配置
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

#### 5. 应用配置表 (app_settings)
```sql
CREATE TABLE app_settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

### 数据模型结构体 (Rust)

```rust
use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Conversation {
    pub id: String,
    pub title: String,
    pub model_provider: String,
    pub model_name: String,
    pub system_prompt: Option<String>,
    pub temperature: f64,
    pub max_tokens: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: String,
    pub conversation_id: String,
    pub role: String,
    pub content: String,
    pub token_count: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProviderConfig {
    pub id: String,
    pub provider_name: String,
    pub api_key: String,  // 加密存储
    pub base_url: Option<String>,
    pub is_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct McpServer {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub endpoint: String,
    pub is_enabled: bool,
    pub config_json: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AppSetting {
    pub key: String,
    pub value: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

## API架构设计

### Tauri命令接口

#### 1. 会话管理API
```rust
#[tauri::command]
async fn create_conversation(
    title: String,
    model_provider: String,
    model_name: String,
    system_prompt: Option<String>,
    state: State<'_, AppState>,
) -> Result<Conversation, String> {
    let db = &state.db;
    let conversation = Conversation {
        id: Uuid::new_v4().to_string(),
        title,
        model_provider,
        model_name,
        system_prompt,
        temperature: 0.7,
        max_tokens: 2048,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    let query = sqlx::query!(
        "INSERT INTO conversations (id, title, model_provider, model_name, system_prompt, temperature, max_tokens, created_at, updated_at) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        conversation.id,
        conversation.title,
        conversation.model_provider,
        conversation.model_name,
        conversation.system_prompt,
        conversation.temperature,
        conversation.max_tokens,
        conversation.created_at,
        conversation.updated_at
    );
    
    query.execute(db).await.map_err(|e| e.to_string())?;
    Ok(conversation)
}

#[tauri::command]
async fn get_conversations(state: State<'_, AppState>) -> Result<Vec<Conversation>, String> {
    let db = &state.db;
    let conversations = sqlx::query_as::<_, Conversation>(
        "SELECT * FROM conversations ORDER BY updated_at DESC"
    )
    .fetch_all(db)
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(conversations)
}

#[tauri::command]
async fn delete_conversation(
    conversation_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = &state.db;
    sqlx::query!("DELETE FROM conversations WHERE id = ?", conversation_id)
        .execute(db)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}
```

#### 2. 消息管理API
```rust
#[tauri::command]
async fn send_message(
    conversation_id: String,
    content: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let db = &state.db;
    let llm_service = &state.llm_service;
    
    // 保存用户消息
    let user_message = Message {
        id: Uuid::new_v4().to_string(),
        conversation_id: conversation_id.clone(),
        role: "user".to_string(),
        content: content.clone(),
        token_count: None,
        created_at: Utc::now(),
    };
    
    save_message(db, &user_message).await?;
    
    // 获取会话信息
    let conversation = get_conversation_by_id(db, &conversation_id).await?;
    
    // 调用LLM API
    let response = llm_service.send_message(&conversation, &content).await?;
    
    // 保存AI回复
    let ai_message = Message {
        id: Uuid::new_v4().to_string(),
        conversation_id,
        role: "assistant".to_string(),
        content: response.clone(),
        token_count: None,
        created_at: Utc::now(),
    };
    
    save_message(db, &ai_message).await?;
    
    Ok(response)
}

#[tauri::command]
async fn get_messages(
    conversation_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<Message>, String> {
    let db = &state.db;
    let messages = sqlx::query_as::<_, Message>(
        "SELECT * FROM messages WHERE conversation_id = ? ORDER BY created_at ASC"
    )
    .bind(conversation_id)
    .fetch_all(db)
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(messages)
}
```

#### 3. 流式响应API
```rust
#[tauri::command]
async fn send_message_stream(
    conversation_id: String,
    content: String,
    window: Window,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = &state.db;
    let llm_service = &state.llm_service;
    
    // 保存用户消息
    let user_message = Message {
        id: Uuid::new_v4().to_string(),
        conversation_id: conversation_id.clone(),
        role: "user".to_string(),
        content: content.clone(),
        token_count: None,
        created_at: Utc::now(),
    };
    
    save_message(db, &user_message).await?;
    
    // 获取会话信息
    let conversation = get_conversation_by_id(db, &conversation_id).await?;
    
    // 创建流式响应
    let mut response_content = String::new();
    let mut stream = llm_service.send_message_stream(&conversation, &content).await?;
    
    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(text) => {
                response_content.push_str(&text);
                // 发送流式数据到前端
                window.emit("message_chunk", &text).map_err(|e| e.to_string())?;
            }
            Err(e) => {
                window.emit("message_error", &e.to_string()).map_err(|e| e.to_string())?;
                return Err(e.to_string());
            }
        }
    }
    
    // 保存完整的AI回复
    let ai_message = Message {
        id: Uuid::new_v4().to_string(),
        conversation_id,
        role: "assistant".to_string(),
        content: response_content,
        token_count: None,
        created_at: Utc::now(),
    };
    
    save_message(db, &ai_message).await?;
    window.emit("message_complete", &ai_message).map_err(|e| e.to_string())?;
    
    Ok(())
}
```

#### 4. 提供商配置API
```rust
#[tauri::command]
async fn save_provider_config(
    provider_name: String,
    api_key: String,
    base_url: Option<String>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = &state.db;
    let crypto_service = &state.crypto_service;
    
    // 加密API密钥
    let encrypted_key = crypto_service.encrypt(&api_key).map_err(|e| e.to_string())?;
    
    sqlx::query!(
        "INSERT OR REPLACE INTO provider_configs (id, provider_name, api_key, base_url, is_enabled, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
        Uuid::new_v4().to_string(),
        provider_name,
        encrypted_key,
        base_url,
        true,
        Utc::now(),
        Utc::now()
    )
    .execute(db)
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
async fn get_provider_configs(state: State<'_, AppState>) -> Result<Vec<ProviderConfig>, String> {
    let db = &state.db;
    let crypto_service = &state.crypto_service;
    
    let mut configs = sqlx::query_as::<_, ProviderConfig>(
        "SELECT * FROM provider_configs ORDER BY provider_name"
    )
    .fetch_all(db)
    .await
    .map_err(|e| e.to_string())?;
    
    // 解密API密钥
    for config in &mut configs {
        config.api_key = crypto_service.decrypt(&config.api_key).map_err(|e| e.to_string())?;
    }
    
    Ok(configs)
}
```

### LLM服务抽象层 (使用Rig库)

```rust
use rig::{Client, Provider, Model};
use tokio_stream::Stream;

pub struct LLMService {
    clients: HashMap<String, Box<dyn LLMProvider>>,
}

#[async_trait::async_trait]
pub trait LLMProvider: Send + Sync {
    async fn send_message(&self, conversation: &Conversation, content: &str) -> Result<String, LLMError>;
    async fn send_message_stream(&self, conversation: &Conversation, content: &str) -> Result<Pin<Box<dyn Stream<Item = Result<String, LLMError>> + Send>>, LLMError>;
}

pub struct OpenAIProvider {
    client: rig::providers::openai::Client,
}

impl OpenAIProvider {
    pub fn new(api_key: &str) -> Self {
        let client = rig::providers::openai::Client::new(api_key);
        Self { client }
    }
}

#[async_trait::async_trait]
impl LLMProvider for OpenAIProvider {
    async fn send_message(&self, conversation: &Conversation, content: &str) -> Result<String, LLMError> {
        let model = self.client.model(&conversation.model_name);
        
        let response = model
            .chat()
            .temperature(conversation.temperature)
            .max_tokens(conversation.max_tokens)
            .system_prompt(conversation.system_prompt.as_deref().unwrap_or(""))
            .user_message(content)
            .send()
            .await?;
        
        Ok(response.content)
    }
    
    async fn send_message_stream(&self, conversation: &Conversation, content: &str) -> Result<Pin<Box<dyn Stream<Item = Result<String, LLMError>> + Send>>, LLMError> {
        let model = self.client.model(&conversation.model_name);
        
        let stream = model
            .chat()
            .temperature(conversation.temperature)
            .max_tokens(conversation.max_tokens)
            .system_prompt(conversation.system_prompt.as_deref().unwrap_or(""))
            .user_message(content)
            .stream()
            .await?;
        
        Ok(Box::pin(stream))
    }
}

impl LLMService {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }
    
    pub fn add_provider(&mut self, name: String, provider: Box<dyn LLMProvider>) {
        self.clients.insert(name, provider);
    }
    
    pub async fn send_message(&self, conversation: &Conversation, content: &str) -> Result<String, LLMError> {
        let provider = self.clients.get(&conversation.model_provider)
            .ok_or_else(|| LLMError::ProviderNotFound(conversation.model_provider.clone()))?;
        
        provider.send_message(conversation, content).await
    }
    
    pub async fn send_message_stream(&self, conversation: &Conversation, content: &str) -> Result<Pin<Box<dyn Stream<Item = Result<String, LLMError>> + Send>>, LLMError> {
        let provider = self.clients.get(&conversation.model_provider)
            .ok_or_else(|| LLMError::ProviderNotFound(conversation.model_provider.clone()))?;
        
        provider.send_message_stream(conversation, content).await
    }
}
``` 