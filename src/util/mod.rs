use config::Config;
use redis::Client as RedisClient;
use sea_orm::DatabaseConnection;

pub mod auth;
pub mod helper;

#[derive(Debug, Clone)]
pub struct AppState {
    pub cfg: Config,
    pub db: DatabaseConnection,
    pub redis: RedisClient,
}
