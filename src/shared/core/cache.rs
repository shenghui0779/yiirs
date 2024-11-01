use config::Config;
use std::io;
use std::{sync::OnceLock, time::Duration};

type RedisPool = r2d2::Pool<redis::Client>;
type RedisAsyncPool = mobc::Pool<RedisAsyncConnManager>;
type RedisClusterPool = r2d2::Pool<redis::cluster::ClusterClient>;
type RedisClusterAsyncPool = mobc::Pool<RedisClusterAsyncConnManager>;

static REDIS_POOL: OnceLock<RedisPool> = OnceLock::new();
static REDIS_ASYNC_POOL: OnceLock<RedisAsyncPool> = OnceLock::new();
static REDIS_CLUSTER_POOL: OnceLock<RedisClusterPool> = OnceLock::new();
static REDIS_CLUSTER_ASYNC_POOL: OnceLock<RedisClusterAsyncPool> = OnceLock::new();

pub fn init_redis(cfg: &Config) {
    let (pool, async_pool) =
        new_redis(cfg, "redis").unwrap_or_else(|e| panic!("Redis连接失败: {}", e));
    let _ = REDIS_POOL.set(pool);
    let _ = REDIS_ASYNC_POOL.set(async_pool);
}

pub fn redis_pool() -> &'static RedisPool {
    REDIS_POOL
        .get()
        .unwrap_or_else(|| panic!("Redis连接池未初始化"))
}

pub fn init_redis_cluster(cfg: &Config) {
    let (pool, async_pool) = new_redis_cluster(cfg, "redis-cluster")
        .unwrap_or_else(|e| panic!("Redis集群连接失败: {}", e));
    let _ = REDIS_CLUSTER_POOL.set(pool);
    let _ = REDIS_CLUSTER_ASYNC_POOL.set(async_pool);
}

pub fn redis_async_pool() -> &'static RedisAsyncPool {
    REDIS_ASYNC_POOL
        .get()
        .unwrap_or_else(|| panic!("Redis异步连接池未初始化"))
}

pub fn redis_cluster_pool() -> &'static RedisClusterPool {
    REDIS_CLUSTER_POOL
        .get()
        .unwrap_or_else(|| panic!("Redis集群连接池未初始化"))
}

pub fn redis_cluster_async_pool() -> &'static RedisClusterAsyncPool {
    REDIS_CLUSTER_ASYNC_POOL
        .get()
        .unwrap_or_else(|| panic!("Redis集群异步连接池未初始化"))
}

pub fn new_redis(cfg: &Config, key: &str) -> anyhow::Result<(RedisPool, RedisAsyncPool)> {
    let client = redis::Client::open(cfg.get_string(&format!("{}.dsn", key))?)?;
    let mut conn = client.get_connection()?;
    let _ = redis::cmd("PING").query::<String>(&mut conn)?;

    let max_size = cfg
        .get_int(&format!("{}.options.max_size", key))
        .unwrap_or(20);
    let min_idle = cfg.get_int("{}.options.min_idle").unwrap_or(10);
    let conn_timeout = cfg.get_int("{}.options.conn_timeout").unwrap_or(10);
    let idle_timeout = cfg.get_int("{}.options.idle_timeout").unwrap_or(300);
    let max_lifetime = cfg.get_int("{}.options.max_lifetime").unwrap_or(600);

    // 同步
    let pool = r2d2::Pool::builder()
        .max_size(max_size as u32)
        .min_idle(Some(min_idle as u32))
        .connection_timeout(Duration::from_secs(conn_timeout as u64))
        .idle_timeout(Some(Duration::from_secs(idle_timeout as u64)))
        .max_lifetime(Some(Duration::from_secs(max_lifetime as u64)))
        .build(client.clone())?;

    // 异步
    let async_pool = mobc::Pool::builder()
        .max_open(max_size as u64)
        .max_idle(min_idle as u64)
        .get_timeout(Some(Duration::from_secs(conn_timeout as u64)))
        .max_idle_lifetime(Some(Duration::from_secs(idle_timeout as u64)))
        .max_lifetime(Some(Duration::from_secs(max_lifetime as u64)))
        .build(RedisAsyncConnManager::new(client));

    Ok((pool, async_pool))
}

pub fn new_redis_cluster(
    cfg: &Config,
    key: &str,
) -> anyhow::Result<(RedisClusterPool, RedisClusterAsyncPool)> {
    let nodes = cfg
        .get_array(&format!("{}.nodes", key))?
        .into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();
    let client = redis::cluster::ClusterClient::new(nodes)?;
    let mut conn = client.get_connection()?;
    let _ = redis::cmd("PING").query::<String>(&mut conn)?;

    let max_size = cfg
        .get_int(&format!("{}.options.max_size", key))
        .unwrap_or(20);
    let min_idle = cfg.get_int("{}.options.min_idle").unwrap_or(10);
    let conn_timeout = cfg.get_int("{}.options.conn_timeout").unwrap_or(10);
    let idle_timeout = cfg.get_int("{}.options.idle_timeout").unwrap_or(300);
    let max_lifetime = cfg.get_int("{}.options.max_lifetime").unwrap_or(600);

    // 同步
    let pool = r2d2::Pool::builder()
        .max_size(max_size as u32)
        .min_idle(Some(min_idle as u32))
        .connection_timeout(Duration::from_secs(conn_timeout as u64))
        .idle_timeout(Some(Duration::from_secs(idle_timeout as u64)))
        .max_lifetime(Some(Duration::from_secs(max_lifetime as u64)))
        .build(client.clone())?;

    // 异步
    let async_pool = mobc::Pool::builder()
        .max_open(max_size as u64)
        .max_idle(min_idle as u64)
        .get_timeout(Some(Duration::from_secs(conn_timeout as u64)))
        .max_idle_lifetime(Some(Duration::from_secs(idle_timeout as u64)))
        .max_lifetime(Some(Duration::from_secs(max_lifetime as u64)))
        .build(RedisClusterAsyncConnManager::new(client));

    Ok((pool, async_pool))
}

pub struct RedisAsyncConnManager {
    client: redis::Client,
}

impl RedisAsyncConnManager {
    pub fn new(c: redis::Client) -> Self {
        Self { client: c }
    }
}

#[mobc::async_trait]
impl mobc::Manager for RedisAsyncConnManager {
    type Connection = redis::aio::MultiplexedConnection;
    type Error = redis::RedisError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let c = self.client.get_multiplexed_async_connection().await?;
        Ok(c)
    }

    async fn check(&self, mut conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        if redis::cmd("PING")
            .query_async::<()>(&mut conn)
            .await
            .is_err()
        {
            return Err(redis::RedisError::from(io::Error::from(
                io::ErrorKind::BrokenPipe,
            )));
        }
        Ok(conn)
    }
}

pub struct RedisClusterAsyncConnManager {
    client: redis::cluster::ClusterClient,
}

impl RedisClusterAsyncConnManager {
    pub fn new(c: redis::cluster::ClusterClient) -> Self {
        Self { client: c }
    }
}

#[mobc::async_trait]
impl mobc::Manager for RedisClusterAsyncConnManager {
    type Connection = redis::cluster_async::ClusterConnection;
    type Error = redis::RedisError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let c = self.client.get_async_connection().await?;
        Ok(c)
    }

    async fn check(&self, mut conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        if redis::cmd("PING")
            .query_async::<()>(&mut conn)
            .await
            .is_err()
        {
            return Err(redis::RedisError::from(io::Error::from(
                io::ErrorKind::BrokenPipe,
            )));
        }
        Ok(conn)
    }
}
