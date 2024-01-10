use config::Config;
use redis::{cluster::ClusterClient, Client};
use std::{sync::OnceLock, time::Duration};

static REDIS: OnceLock<Client> = OnceLock::new();
static REDIS_CLUSTER: OnceLock<ClusterClient> = OnceLock::new();

pub fn init(cfg: &Config) {
    let client = Client::open(cfg.get_string("redis.dsn").expect("缺少DSN配置"))
        .unwrap_or_else(|e| panic!("Redis连接失败：{}", e));

    let mut conn = client
        .get_connection_with_timeout(Duration::from_secs(10))
        .unwrap_or_else(|e| panic!("Redis连接失败：{}", e));
    let _ = redis::cmd("PING")
        .query::<String>(&mut conn)
        .unwrap_or_else(|e| panic!("Redis连接失败：{}", e));

    let _ = REDIS.set(client);
}

pub fn init_cluster(cfg: &Config) {
    let nodes = cfg
        .get_array("redis-cluster.nodes")
        .expect("缺少nodes配置")
        .into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    let client = ClusterClient::new(nodes).unwrap_or_else(|e| panic!("Redis连接失败：{}", e));

    let mut conn = client
        .get_connection()
        .unwrap_or_else(|e| panic!("Redis集群连接失败：{}", e));
    let _ = redis::cmd("PING")
        .query::<String>(&mut conn)
        .unwrap_or_else(|e| panic!("Redis集群连接失败：{}", e));

    let _ = REDIS_CLUSTER.set(client);
}

pub fn client() -> &'static Client {
    REDIS.get().unwrap_or_else(|| panic!("Redis连接未初始化"))
}

pub fn cluster() -> &'static ClusterClient {
    REDIS_CLUSTER
        .get()
        .unwrap_or_else(|| panic!("Redis集群未初始化"))
}
