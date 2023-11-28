use std::time::Duration;

use config::Config;

pub fn init(cfg: &Config) -> redis::Client {
    let client = redis::Client::open(cfg.get_string("redis.dsn").expect("缺少DSN配置"))
        .unwrap_or_else(|e| panic!("Redis连接失败：{}", e));

    let mut conn = client
        .get_connection_with_timeout(Duration::from_secs(10))
        .unwrap_or_else(|e| panic!("Redis连接失败：{}", e));
    let _ = redis::cmd("PING")
        .query::<String>(&mut conn)
        .unwrap_or_else(|e| panic!("Redis连接失败：{}", e));

    client
}
