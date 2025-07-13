# 技术上下文

## 技术栈架构

### 前端技术栈
- **框架**: React 18.x
- **类型系统**: TypeScript 5.x
- **构建工具**: Vite 5.x
- **UI库**: Tailwind CSS + Headless UI
- **状态管理**: Zustand / Redux Toolkit
- **路由**: React Router v6
- **表单处理**: React Hook Form + Zod
- **动画**: Framer Motion
- **图标**: Lucide React

### 后端技术栈（Tauri）
- **框架**: Tauri 2.x
- **语言**: Rust
- **HTTP客户端**: reqwest
- **序列化**: serde
- **数据库**: SQLite (rusqlite)
- **加密**: ring / rustcrypto
- **配置管理**: tauri-plugin-store

### 开发工具
- **包管理**: pnpm
- **代码格式化**: Prettier
- **代码检查**: ESLint
- **类型检查**: TypeScript
- **测试**: Vitest + React Testing Library
- **版本控制**: Git
- **IDE**: VS Code

## 核心依赖库

### 前端依赖
```json
{
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "react-router-dom": "^6.8.0",
    "@tauri-apps/api": "^2.0.0",
    "@tauri-apps/plugin-store": "^2.0.0",
    "zustand": "^4.4.0",
    "react-hook-form": "^7.43.0",
    "zod": "^3.20.0",
    "@hookform/resolvers": "^2.9.0",
    "tailwindcss": "^3.3.0",
    "@headlessui/react": "^1.7.0",
    "framer-motion": "^10.0.0",
    "lucide-react": "^0.220.0",
    "react-markdown": "^8.0.0",
    "prismjs": "^1.29.0",
    "react-syntax-highlighter": "^15.5.0"
  },
  "devDependencies": {
    "@types/react": "^18.0.0",
    "@types/react-dom": "^18.0.0",
    "@vitejs/plugin-react": "^4.0.0",
    "typescript": "^5.0.0",
    "vite": "^5.0.0",
    "eslint": "^8.0.0",
    "prettier": "^2.8.0",
    "vitest": "^0.34.0",
    "@testing-library/react": "^13.0.0",
    "@testing-library/jest-dom": "^5.16.0",
    "@tailwindcss/typography": "^0.5.0"
  }
}
```

### Rust依赖
```toml
[dependencies]
tauri = { version = "2.0", features = ["api-all"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
reqwest = { version = "0.11", features = ["json", "stream"] }
sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "sqlite", "uuid", "chrono", "migrate"] }
rig = "0.1"
uuid = { version = "1.3", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1.0"
thiserror = "1.0"
ring = "0.16"
base64 = "0.21"
dirs = "5.0"
```

## 核心技术选择

### 数据存储层
- **SQLx**: 异步SQL工具包，支持编译时SQL验证
- **SQLite**: 嵌入式数据库，支持本地存储
- **数据库迁移**: 使用sqlx migrate进行版本控制

### LLM交互层
- **Rig**: 现代化的LLM交互库
- **特性**: 
  - 多提供商支持（OpenAI、Anthropic、Google等）
  - 流式响应处理
  - 错误重试机制
  - 异步处理

## LLM服务商集成

### OpenAI
- **API**: GPT-4, GPT-3.5-turbo
- **库**: openai-api (通过HTTP)
- **认证**: API Key
- **流式响应**: Server-Sent Events

### Anthropic
- **API**: Claude-3系列
- **库**: anthropic-api (通过HTTP)
- **认证**: API Key
- **流式响应**: Server-Sent Events

### Google
- **API**: Gemini Pro
- **库**: google-ai-generativelanguage
- **认证**: API Key
- **流式响应**: 支持

### 百度
- **API**: 文心一言
- **库**: 百度智能云API
- **认证**: API Key + Secret Key
- **流式响应**: 支持

### 本地模型
- **Ollama**: 本地模型运行
- **LM Studio**: 本地模型服务
- **接口**: OpenAI兼容API

## MCP Server支持

### MCP协议
- **协议版本**: 1.0
- **传输层**: JSON-RPC 2.0
- **通信方式**: WebSocket / HTTP

### 配置管理
- **配置文件**: JSON格式
- **热重载**: 支持
- **验证**: Schema验证

### 服务器类型
- **文件系统**: 文件操作MCP服务器
- **数据库**: 数据库查询MCP服务器
- **网络**: HTTP请求MCP服务器
- **自定义**: 用户自定义MCP服务器

## 数据存储

### 本地数据库
- **引擎**: SQLite
- **表设计**:
  - conversations: 会话记录
  - messages: 消息记录
  - configurations: 配置信息
  - mcp_servers: MCP服务器配置

### 配置存储
- **引擎**: Tauri Store Plugin
- **位置**: 用户数据目录
- **加密**: 敏感数据加密存储

## 安全考虑

### API密钥管理
- **存储**: 加密存储在本地
- **传输**: HTTPS加密传输
- **访问**: 内存中临时存储

### 数据安全
- **本地存储**: 聊天记录本地存储
- **加密**: 敏感数据AES加密
- **权限**: 最小权限原则

## 性能优化

### 前端优化
- **懒加载**: 路由和组件懒加载
- **虚拟滚动**: 长列表虚拟滚动
- **状态管理**: 精确的状态订阅
- **缓存**: 适当的数据缓存

### 后端优化
- **连接池**: HTTP连接复用
- **异步处理**: 全异步架构
- **内存管理**: 适当的内存管理
- **数据库**: 索引优化

## 构建配置

### 开发环境
- **热重载**: Vite HMR
- **代理**: API代理配置
- **环境变量**: 开发环境变量

### 生产构建
- **优化**: 代码分割和压缩
- **打包**: Tauri打包配置
- **签名**: 代码签名配置
- **分发**: 自动化分发流程 