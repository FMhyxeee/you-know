use std::collections::HashMap;
use tauri::Manager;
use sqlx::{migrate::MigrateDatabase, Row, SqlitePool};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// 数据模型
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Message {
    pub id: String,
    pub conversation_id: String,
    pub role: String,
    pub content: String,
    pub token_count: Option<i32>,
    pub created_at: DateTime<Utc>,
}

// 应用状态
#[derive(Debug)]
pub struct AppState {
    pub db: SqlitePool,
}

// 技术验证：数据库连接测试
#[tauri::command]
async fn test_database_connection(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let db = &state.db;
    
    // 测试数据库连接
    let row = sqlx::query("SELECT 'Database connection successful' as message")
        .fetch_one(db)
        .await
        .map_err(|e| e.to_string())?;
    
    let message: String = row.get("message");
    Ok(message)
}

// 技术验证：创建表结构
#[tauri::command]
async fn create_tables(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let db = &state.db;
    
    // 创建conversations表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS conversations (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            model_provider TEXT NOT NULL,
            model_name TEXT NOT NULL,
            system_prompt TEXT,
            temperature REAL DEFAULT 0.7,
            max_tokens INTEGER DEFAULT 2048,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(db)
    .await
    .map_err(|e| e.to_string())?;
    
    // 创建messages表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS messages (
            id TEXT PRIMARY KEY,
            conversation_id TEXT NOT NULL,
            role TEXT NOT NULL,
            content TEXT NOT NULL,
            token_count INTEGER,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (conversation_id) REFERENCES conversations(id) ON DELETE CASCADE
        )
        "#
    )
    .execute(db)
    .await
    .map_err(|e| e.to_string())?;
    
    // 创建provider_configs表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS provider_configs (
            id TEXT PRIMARY KEY,
            provider_name TEXT NOT NULL UNIQUE,
            api_key TEXT NOT NULL,
            base_url TEXT,
            is_enabled BOOLEAN DEFAULT TRUE,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(db)
    .await
    .map_err(|e| e.to_string())?;
    
    Ok("Tables created successfully".to_string())
}

// 技术验证：基本CRUD操作
#[tauri::command]
async fn test_crud_operations(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let db = &state.db;
    
    // 插入测试数据
    let conversation_id = Uuid::new_v4().to_string();
    let now = Utc::now();
    
    sqlx::query(
        "INSERT INTO conversations (id, title, model_provider, model_name, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&conversation_id)
    .bind("Test Conversation")
    .bind("openai")
    .bind("gpt-4")
    .bind(now.to_rfc3339())
    .bind(now.to_rfc3339())
    .execute(db)
    .await
    .map_err(|e| e.to_string())?;
    
    // 查询测试数据
    let row = sqlx::query("SELECT COUNT(*) as count FROM conversations")
        .fetch_one(db)
        .await
        .map_err(|e| e.to_string())?;
    
    let count: i64 = row.get("count");
    
    // 清理测试数据
    sqlx::query("DELETE FROM conversations WHERE id = ?")
        .bind(&conversation_id)
        .execute(db)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(format!("CRUD operations successful. Found {} conversations", count))
}


// 初始化数据库
async fn init_database() -> Result<SqlitePool, Box<dyn std::error::Error>> {
    // 获取应用数据目录
    let app_data_dir = dirs::data_dir()
        .ok_or("Unable to find data directory")?
        .join("you-know");
    
    // 确保目录存在
    std::fs::create_dir_all(&app_data_dir)?;
    
    // 数据库文件路径, 如果没有就创建
    let db_path = app_data_dir.join("app.db");
    if !db_path.exists() {
        sqlx::Sqlite::create_database(db_path.as_path().to_str().unwrap()).await.unwrap();
    }
    let database_url = format!("sqlite:{}", db_path.display());
    
    // 创建连接池
    let pool = SqlitePool::connect(&database_url).await?;
    
    // 运行迁移（如果需要）
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    Ok(pool)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // 初始化数据库
            let db = tauri::async_runtime::block_on(async {
                init_database().await.expect("Failed to initialize database")
            });
            
            // 设置应用状态
            app.manage(AppState { db });
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            test_database_connection,
            create_tables,
            test_crud_operations,
            greet
        ])
        .on_window_event(|window, f| {
            match f {
                tauri::WindowEvent::CloseRequested { ..} => {

                    #[cfg(debug_assertions)]
                    // clear db file , only when develop mode
                    let db_path = window.app_handle().path().app_data_dir().unwrap().join("app.db");
                    if db_path.exists() {
                        std::fs::remove_file(db_path).expect("Failed to remove db file");
                    }
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// 保留原有的greet函数用于基本测试
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    run();
}
