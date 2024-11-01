use std::{sync::OnceLock, time::Duration};

use config::Config;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

static DB: OnceLock<DatabaseConnection> = OnceLock::new();

pub async fn init(cfg: &Config) {
    let conn = new(cfg, "db")
        .await
        .unwrap_or_else(|e| panic!("数据库连接失败：{}", e));
    let _ = DB.set(conn);
}

pub fn conn() -> &'static DatabaseConnection {
    DB.get().unwrap_or_else(|| panic!("数据库连接未初始化"))
}

pub async fn new(cfg: &Config, key: &str) -> anyhow::Result<DatabaseConnection> {
    let mut opt = ConnectOptions::new(cfg.get_string(&format!("{}.dsn", key))?);

    opt.min_connections(
        cfg.get_int(&format!("{}.options.min_conns", key))
            .unwrap_or(10) as u32,
    )
    .max_connections(
        cfg.get_int(&format!("{}.options.max_conns", key))
            .unwrap_or(20) as u32,
    )
    .connect_timeout(Duration::from_secs(
        cfg.get_int(&format!("{}.options.conn_timeout", key))
            .unwrap_or(10) as u64,
    ))
    .idle_timeout(Duration::from_secs(
        cfg.get_int(&format!("{}.options.idle_timeout", key))
            .unwrap_or(300) as u64,
    ))
    .max_lifetime(Duration::from_secs(
        cfg.get_int(&format!("{}.options.max_lifetime", key))
            .unwrap_or(600) as u64,
    ))
    .sqlx_logging(cfg.get_bool("app.debug").unwrap_or_default());

    let conn = Database::connect(opt).await?;
    let _ = conn.ping().await?;

    Ok(conn)
}
