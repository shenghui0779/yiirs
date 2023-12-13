use config::Config;
use redis::Client as RedisClient;
use sea_orm::DatabaseConnection;

pub mod auth;
pub mod helper;
pub mod time;

#[derive(Debug, Clone)]
pub struct AppState {
    pub cfg: Config,
    pub db: DatabaseConnection,
    pub redis: RedisClient,
}

pub async fn app_state(cfg: Config) -> AppState {
    let db = crate::db::init(&cfg).await;
    let redis = crate::redis::init_client(&cfg);

    AppState { cfg, db, redis }
}
