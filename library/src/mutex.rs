use nanoid::nanoid;
use rand::Rng;
use redis::{AsyncCommands, Commands, ExistenceCheck::NX, SetExpiry::PX, SetOptions};

// 基于Redis的分布式锁
pub struct Distributed<'a> {
    cli: &'a redis::Client,
    key: String,
    token: String,
    expire: usize,
}

impl<'a> Distributed<'a> {
    pub fn new(cli: &redis::Client, key: String, ttl: chrono::Duration) -> Distributed {
        Distributed {
            cli,
            key,
            token: String::from(""),
            expire: ttl.num_milliseconds() as usize,
        }
    }

    // 获取锁
    pub async fn lock(&mut self) -> Result<bool, redis::RedisError> {
        return self._acquire().await;
    }

    // 尝试获取锁
    pub async fn try_lock(&mut self, attempts: i32) -> Result<bool, redis::RedisError> {
        for _ in 1..attempts {
            match self._acquire().await {
                Ok(v) => {
                    if v {
                        return Ok(true);
                    }
                }
                Err(e) => return Err(e),
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(
                rand::thread_rng().gen_range(50..=200),
            ))
            .await;
        }

        return Ok(false);
    }

    async fn _acquire(&mut self) -> Result<bool, redis::RedisError> {
        let mut conn = self
            .cli
            .get_multiplexed_async_connection_with_timeouts(
                std::time::Duration::from_secs(10),
                std::time::Duration::from_secs(10),
            )
            .await?;
        let opts = SetOptions::default()
            .conditional_set(NX)
            .with_expiration(PX(self.expire));
        let token = nanoid!(32);
        let ret = conn
            .set_options::<&String, &String, bool>(&self.key, &token, opts)
            .await;
        match ret {
            Ok(v) => {
                if v {
                    self.token = token;
                    return Ok(true);
                }
                return Ok(false);
            }
            Err(e) => {
                // 尝试GET一次：避免因redis网络错误导致误加锁
                match conn.get::<&String, redis::Value>(&self.key).await {
                    Ok(v) => match v {
                        redis::Value::Nil => return Err(e),
                        _ => {
                            if redis::from_redis_value::<String>(&v).unwrap_or_default() == token {
                                self.token = token;
                                return Ok(true);
                            }
                            return Ok(false);
                        }
                    },
                    Err(e) => return Err(e),
                }
            }
        }
    }
}

// 释放锁
impl<'a> Drop for Distributed<'a> {
    fn drop(&mut self) {
        if self.token.len() == 0 {
            return;
        }

        let mut conn = match self.cli.get_connection() {
            Ok(v) => v,
            Err(e) => {
                tracing::error!(error = ?e, "[mutex] redis get connection error");
                return;
            }
        };

        match conn.get::<&String, redis::Value>(&self.key) {
            Ok(v) => match v {
                redis::Value::Nil => (),
                _ => {
                    if redis::from_redis_value::<String>(&v).unwrap_or_default() == self.token {
                        if let Err(e) = conn.del::<&String, ()>(&self.key) {
                            tracing::error!(error = ?e, "[mutex] redis del key({}) error", self.key);
                        }
                    }
                }
            },
            Err(e) => {
                tracing::error!(error = ?e, "[mutex] redis get key({}) error", self.key);
            }
        }
    }
}
