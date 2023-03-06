use std::{env, time::Duration};

use once_cell::sync::OnceCell;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

static DB: OnceCell<DatabaseConnection> = OnceCell::new();

pub async fn init(debug: bool) {
    let dsn = env::var("DATABASE_URL").expect("缺少配置：DATABASE_URL");
    let min_conns = env::var("DB_MIN_CONNS")
        .expect("缺少配置：DB_MIN_CONNS")
        .parse::<u32>()
        .expect("配置DB_MIN_CONNS必须为整数");
    let max_conns = env::var("DB_MAX_CONNS")
        .expect("缺少配置：DB_MAX_CONNS")
        .parse::<u32>()
        .expect("配置DB_MAX_CONNS必须为整数");
    let conn_timeout = env::var("DB_CONN_TIMEOUT")
        .expect("缺少配置：DB_CONN_TIMEOUT")
        .parse::<u64>()
        .expect("配置DB_CONN_TIMEOUT必须为整数");
    let idle_timeout = env::var("DB_IDLE_TIMEOUT")
        .expect("缺少配置：DB_IDLE_TIMEOUT")
        .parse::<u64>()
        .expect("配置DB_IDLE_TIMEOUT必须为整数");
    let max_lifetime = env::var("DB_MAX_LIFETIME")
        .expect("缺少配置：DB_MAX_LIFETIME")
        .parse::<u64>()
        .expect("配置DB_MAX_LIFETIME必须为整数");

    let mut opt = ConnectOptions::new(dsn);

    opt.min_connections(min_conns)
        .max_connections(max_conns)
        .connect_timeout(Duration::from_secs(conn_timeout))
        .idle_timeout(Duration::from_secs(idle_timeout))
        .max_lifetime(Duration::from_secs(max_lifetime))
        .sqlx_logging(debug);

    DB.set(Database::connect(opt).await.unwrap()).unwrap()
}

pub fn get() -> &'static DatabaseConnection {
    DB.get().unwrap()
}
