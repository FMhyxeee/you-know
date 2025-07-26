use crate::error::{AppError, AppResult};
use std::path::PathBuf;

/// 获取应用数据目录路径
pub fn get_app_data_dir() -> AppResult<PathBuf> {
    let home_dir =
        dirs::home_dir().ok_or_else(|| AppError::config("Unable to find home directory"))?;
    Ok(home_dir.join(".you-know"))
}

/// 获取数据库文件路径
pub fn get_database_path() -> AppResult<PathBuf> {
    let app_data_dir = get_app_data_dir()?;
    Ok(app_data_dir.join("app.db"))
}

/// 获取数据库连接URL
pub fn get_database_url() -> AppResult<String> {
    let db_path = get_database_path()?;
    Ok(format!("sqlite:{}", db_path.display()))
}

/// 获取日志路径
pub fn get_log_path() -> AppResult<PathBuf> {
    let app_data_dir = get_app_data_dir()?;
    Ok(app_data_dir.join("log").join("app.log"))
}

/// 确保应用数据目录存在
pub fn ensure_app_data_dir() -> AppResult<PathBuf> {
    let app_data_dir = get_app_data_dir()?;
    std::fs::create_dir_all(&app_data_dir)?;
    Ok(app_data_dir)
}
