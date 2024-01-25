use redis::{AsyncCommands, Commands, ExistenceCheck::NX, SetExpiry::PX, SetOptions};

// 基于Redis的分布式锁
pub struct Distributed<'a> {
    cli: &'a redis::Client,
    key: String,
    uuid: String,
    expire: usize,
}

impl<'a> Distributed<'a> {
    pub fn new(
        cli: &redis::Client,
        key: String,
        uuid: String,
        expire: chrono::Duration,
    ) -> Distributed {
        Distributed {
            cli,
            key,
            uuid,
            expire: expire.num_milliseconds() as usize,
        }
    }

    pub async fn lock(
        &self,
        interval: chrono::Duration,
        timeout: chrono::Duration,
    ) -> anyhow::Result<()> {
        let mut conn = self.cli.get_async_connection().await?;
        let ended_at = chrono::Local::now() + timeout;

        while chrono::Local::now() < ended_at {
            let opts = SetOptions::default()
                .conditional_set(NX)
                .with_expiration(PX(self.expire));

            let ok = conn
                .set_options::<&String, &String, bool>(&self.key, &self.uuid, opts)
                .await?;
            if ok {
                return Ok(());
            }

            tokio::time::sleep(interval.to_std()?).await;
        }

        Ok(())
    }
}

impl<'a> Drop for Distributed<'a> {
    fn drop(&mut self) {
        let mut conn = match self.cli.get_connection() {
            Ok(v) => v,
            Err(e) => {
                tracing::error!(error = ?e, "redis get connection error");
                return;
            }
        };

        match conn.get::<&String, redis::Value>(&self.key) {
            Ok(v) => match v {
                redis::Value::Nil => (),
                _ => {
                    if redis::from_redis_value::<String>(&v).unwrap_or_default() == self.uuid {
                        match conn.del::<&String, ()>(&self.key) {
                            Err(e) => {
                                tracing::error!(error = ?e, "redis del key({}) error", self.key);
                            }
                            _ => (),
                        }
                    }
                }
            },
            Err(e) => {
                tracing::error!(error = ?e, "redis get key({}) error", self.key);
            }
        }
    }
}
