use crate::{error::AppResult, utils};
use log::info;
use sqlx::{migrate::MigrateDatabase, SqlitePool};

/// 初始化数据库
pub async fn init_database() -> AppResult<SqlitePool> {
    // 确保应用数据目录存在
    utils::ensure_app_data_dir()?;

    // 获取数据库文件路径
    let db_path = utils::get_database_path()?;

    // 如果数据库文件不存在则创建
    if !db_path.exists() {
        sqlx::Sqlite::create_database(db_path.as_path().to_str().unwrap()).await?;
    }

    // 获取数据库连接URL
    let database_url = utils::get_database_url()?;
    info!("database_url: {:?}", database_url);

    // 创建连接池
    let pool = SqlitePool::connect(&database_url).await?;

    // 运行迁移（如果需要）
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
