use std::time::Duration;

use config::Config;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub mod entity;

pub async fn init(cfg: &Config) -> DatabaseConnection {
    let mut opt = ConnectOptions::new(cfg.get_string("db.dsn").expect("缺少DSN配置"));

    opt.min_connections(cfg.get_int("db.min_conns").unwrap_or(10) as u32)
        .max_connections(cfg.get_int("db.max_conns").unwrap_or(10) as u32)
        .connect_timeout(Duration::from_secs(
            cfg.get_int("db.conn_timeout").unwrap_or(10) as u64,
        ))
        .idle_timeout(Duration::from_secs(
            cfg.get_int("db.idle_timeout").unwrap_or(300) as u64,
        ))
        .max_lifetime(Duration::from_secs(
            cfg.get_int("db.max_lifetime").unwrap_or(600) as u64,
        ))
        .sqlx_logging(cfg.get_bool("app.debug").unwrap_or_default());

    let conn = Database::connect(opt)
        .await
        .unwrap_or_else(|e| panic!("数据库连接失败：{}", e));
    let _ = conn
        .ping()
        .await
        .is_err_and(|e| panic!("数据库连接失败：{}", e));

    conn
}
