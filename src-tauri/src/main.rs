use log::{error, info};
use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};
use you_know_lib::models::AppState;
use you_know_lib::{commands, database, utils};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::Folder {
                        path: utils::get_log_path()
                            .unwrap_or_else(|_| std::path::PathBuf::from("./logs"))
                            .into(),
                        file_name: Some("app.log".to_string()),
                    }),
                ])
                .build(),
        )
        .setup(|app| {
            // 初始化数据库
            let db = tauri::async_runtime::block_on(async {
                match database::init_database().await {
                    Ok(db) => db,
                    Err(e) => {
                        error!("Failed to initialize database: {}", e);
                        panic!("Database initialization failed: {}", e);
                    }
                }
            });

            // 设置应用状态
            app.manage(AppState { db });
            info!("Database initialized successfully");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::test_database_connection,
            commands::create_tables,
            commands::test_crud_operations,
            commands::add_rss_feed,
            commands::get_rss_feeds,
            commands::get_articles,
            commands::get_article_content,
            commands::update_article,
            commands::refresh_rss_feed,
            commands::delete_rss_feed,
            commands::get_statistics,
            commands::greet
        ])
        .on_window_event(|_window, f| {
            match f {
                tauri::WindowEvent::CloseRequested { .. } => {
                    #[cfg(debug_assertions)]
                    {
                        // 在调试模式下删除数据库文件
                        if let Ok(db_path) = utils::get_database_path() {
                            if db_path.exists() {
                                let _ = std::fs::remove_file(db_path);
                            }
                        }
                    }
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    run();
}
