use std::time;

use nanoid::nanoid;
use rand::Rng;
use redis::{AsyncCommands, Commands, ExistenceCheck::NX, SetExpiry::PX};
use tokio::time::{sleep, Duration};

use crate::cache;

// 基于Redis的分布式锁
pub struct RedisLock {
    key: String,
    token: String,
    expire: usize,
}

impl RedisLock {
    pub fn new(key: String, ttl: time::Duration) -> RedisLock {
        RedisLock {
            key,
            token: String::from(""),
            expire: ttl.as_millis() as usize,
        }
    }

    // 获取锁
    pub async fn lock(&mut self) -> anyhow::Result<bool> {
        self._acquire().await
    }

    // 尝试获取锁
    pub async fn try_lock(&mut self, attempts: i32) -> anyhow::Result<bool> {
        for _ in 0..attempts {
            let ok = self._acquire().await?;
            if ok {
                return Ok(true);
            }
            let delay = rand::thread_rng().gen_range(50..=200);
            sleep(Duration::from_millis(delay)).await;
        }
        Ok(false)
    }

    async fn _acquire(&mut self) -> anyhow::Result<bool> {
        let mut conn = cache::redis_async_pool().get().await?;
        let opts = redis::SetOptions::default()
            .conditional_set(NX)
            .with_expiration(PX(self.expire));
        let token = nanoid!(32);

        let ret_setnx: redis::RedisResult<bool> = conn.set_options(&self.key, &token, opts).await;
        match ret_setnx {
            Ok(v) => {
                if v {
                    self.token = token;
                    return Ok(true);
                }
                return Ok(false);
            }
            Err(e) => {
                // 尝试GET一次：避免因redis网络错误导致误加锁
                let ret_get: Option<String> = conn.get(&self.key).await?;
                let v = ret_get.ok_or(e)?;
                if v == token {
                    self.token = token;
                    return Ok(true);
                }
                return Ok(false);
            }
        }
    }
}

// 释放锁
impl Drop for RedisLock {
    fn drop(&mut self) {
        if self.token.len() == 0 {
            return;
        }

        let mut conn = match cache::redis_pool().get() {
            Ok(v) => v,
            Err(e) => {
                tracing::error!(error = ?e, "[mutex] redis get connection error");
                return;
            }
        };

        let ret_get: redis::RedisResult<Option<String>> = conn.get(&self.key);
        match ret_get {
            Ok(v) => {
                if let Some(token) = v {
                    if token == self.token {
                        let ret_del: redis::RedisResult<()> = conn.del(&self.key);
                        if let Err(e) = ret_del {
                            tracing::error!(error = ?e, "[mutex] redis del key({}) error", self.key);
                        }
                    }
                }
            }
            Err(e) => {
                tracing::error!(error = ?e, "[mutex] redis get key({}) error", self.key);
            }
        }
    }
}
