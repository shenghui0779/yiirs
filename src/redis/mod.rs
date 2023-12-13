use config::Config;
use redis::{cluster::ClusterClient, Client};
use std::time::Duration;

pub mod mutex;

pub fn init_client(cfg: &Config) -> Client {
    let client = Client::open(cfg.get_string("redis.dsn").expect("缺少DSN配置"))
        .unwrap_or_else(|e| panic!("Redis连接失败：{}", e));

    let mut conn = client
        .get_connection_with_timeout(Duration::from_secs(10))
        .unwrap_or_else(|e| panic!("Redis连接失败：{}", e));
    let _ = redis::cmd("PING")
        .query::<String>(&mut conn)
        .unwrap_or_else(|e| panic!("Redis连接失败：{}", e));

    client
}

#[allow(dead_code)]
pub fn init_cluster(cfg: &Config) -> ClusterClient {
    let nodes = cfg
        .get_array("redis-cluster.nodes")
        .expect("缺少nodes配置")
        .into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    let client = ClusterClient::new(nodes).unwrap_or_else(|e| panic!("Redis连接失败：{}", e));

    let mut conn = client
        .get_connection()
        .unwrap_or_else(|e| panic!("Redis连接失败：{}", e));
    let _ = redis::cmd("PING")
        .query::<String>(&mut conn)
        .unwrap_or_else(|e| panic!("Redis连接失败：{}", e));

    client
}
