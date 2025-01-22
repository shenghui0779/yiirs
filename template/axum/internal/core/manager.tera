#[derive(Clone)]
pub struct RedisAsyncConnManager {
    client: redis::Client,
}

impl RedisAsyncConnManager {
    pub fn new(c: redis::Client) -> Self {
        Self { client: c }
    }
}

impl bb8::ManageConnection for RedisAsyncConnManager {
    type Connection = redis::aio::MultiplexedConnection;
    type Error = redis::RedisError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        self.client.get_multiplexed_async_connection().await
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        let pong: String = redis::cmd("PING").query_async(conn).await?;
        match pong.as_str() {
            "PONG" => Ok(()),
            _ => Err((redis::ErrorKind::ResponseError, "ping request").into()),
        }
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}

#[derive(Clone)]
pub struct RedisClusterAsyncConnManager {
    client: redis::cluster::ClusterClient,
}

impl RedisClusterAsyncConnManager {
    pub fn new(c: redis::cluster::ClusterClient) -> Self {
        Self { client: c }
    }
}

impl bb8::ManageConnection for RedisClusterAsyncConnManager {
    type Connection = redis::cluster_async::ClusterConnection;
    type Error = redis::RedisError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let c = self.client.get_async_connection().await?;
        Ok(c)
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        let pong: String = redis::cmd("PING").query_async(conn).await?;
        match pong.as_str() {
            "PONG" => Ok(()),
            _ => Err((redis::ErrorKind::ResponseError, "ping request").into()),
        }
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}
