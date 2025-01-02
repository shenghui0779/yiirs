use std::time;

use nanoid::nanoid;
use redis::{AsyncCommands, ExistenceCheck::NX, SetExpiry::PX};
use tokio::time::sleep;

use crate::cache;

pub const SCRIPT: &str = r"
if redis.call('get', KEYS[1]) == ARGV[1] then
    return redis.call('del', KEYS[1])
else
    return 0
end
";

// 基于Redis的分布式锁
pub struct RedisLock<'a> {
    pool: &'a cache::RedisPool,
    async_pool: &'a cache::RedisAsyncPool,
    key: String,
    token: String,
    expire: u64,
    unlock: bool,
}

impl<'a> RedisLock<'a> {
    pub fn new(
        client: (&'a cache::RedisPool, &'a cache::RedisAsyncPool),
        key: String,
        ttl: time::Duration,
        auto_unlock: bool,
    ) -> RedisLock<'a> {
        let (pool, async_pool) = client;
        RedisLock {
            pool,
            async_pool,
            key,
            token: String::from(""),
            expire: ttl.as_millis() as u64,
            unlock: auto_unlock,
        }
    }

    // 获取锁
    pub async fn lock(&mut self) -> anyhow::Result<bool> {
        self._acquire().await
    }

    // 尝试获取锁
    pub async fn try_lock(
        &mut self,
        attempts: i32,
        interval: time::Duration,
    ) -> anyhow::Result<bool> {
        for i in 0..attempts {
            let ok = self._acquire().await?;
            if ok {
                return Ok(true);
            }
            if i < attempts - 1 {
                sleep(interval).await;
            }
        }
        Ok(false)
    }

    // 释放锁(手动)
    pub async fn unlock(&mut self) -> anyhow::Result<()> {
        if self.token.is_empty() {
            return Ok(());
        }
        let conn = self.async_pool.get().await?;
        let script = redis::Script::new(SCRIPT);
        script
            .key(&self.key)
            .arg(&self.token)
            .invoke_async::<()>(&mut conn.into_inner())
            .await?;
        Ok(())
    }

    async fn _acquire(&mut self) -> anyhow::Result<bool> {
        let mut conn = self.async_pool.get().await?;
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
                Ok(false)
            }
            Err(e) => {
                // 尝试GET一次：避免因redis网络错误导致误加锁
                let ret_get: Option<String> = conn.get(&self.key).await?;
                let v = ret_get.ok_or(e)?;
                if v == token {
                    self.token = token;
                    return Ok(true);
                }
                Ok(false)
            }
        }
    }
}

// 释放锁(自动)
impl<'a> Drop for RedisLock<'a> {
    fn drop(&mut self) {
        if !self.unlock || self.token.is_empty() {
            return;
        }

        let mut conn = match self.pool.get() {
            Ok(v) => v,
            Err(e) => {
                tracing::error!(err = ?e, "[mutex] redis get connection error");
                return;
            }
        };

        let script = redis::Script::new(SCRIPT);
        let ret: redis::RedisResult<()> = script.key(&self.key).arg(&self.token).invoke(&mut conn);
        if let Err(e) = ret {
            tracing::error!(err = ?e, "[mutex] redis del key({}) error", self.key);
        }
    }
}
