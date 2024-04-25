use std::{sync::OnceLock, time::Duration};

use config::Config;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

static DB: OnceLock<DatabaseConnection> = OnceLock::new();

pub async fn init(cfg: &Config) {
    let mut opt = ConnectOptions::new(cfg.get_string("db.dsn").expect("缺少DSN配置"));

    opt.min_connections(cfg.get_int("db.options.min_conns").unwrap_or(10) as u32)
        .max_connections(cfg.get_int("db.options.max_conns").unwrap_or(20) as u32)
        .connect_timeout(Duration::from_secs(
            cfg.get_int("db.options.conn_timeout").unwrap_or(10) as u64,
        ))
        .idle_timeout(Duration::from_secs(
            cfg.get_int("db.options.idle_timeout").unwrap_or(300) as u64,
        ))
        .max_lifetime(Duration::from_secs(
            cfg.get_int("db.options.max_lifetime").unwrap_or(600) as u64,
        ))
        .sqlx_logging(cfg.get_bool("app.debug").unwrap_or_default());

    let conn = Database::connect(opt)
        .await
        .unwrap_or_else(|e| panic!("数据库连接失败：{}", e));
    let _ = conn
        .ping()
        .await
        .is_err_and(|e| panic!("数据库连接失败：{}", e));

    let _ = DB.set(conn);
}

pub fn conn() -> &'static DatabaseConnection {
    DB.get().unwrap_or_else(|| panic!("数据库连接未初始化"))
}
