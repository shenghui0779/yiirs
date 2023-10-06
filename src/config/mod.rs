pub mod db;
pub mod logger;

use anyhow::Context;
use sea_orm::DatabaseConnection;
use std::{env, fs};
use tracing_appender::non_blocking::WorkerGuard;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

pub async fn init() -> (WorkerGuard, DatabaseConnection) {
    // cargo run -- /data/config/.env
    let envfile = env::args().nth(1).unwrap_or(String::from(".env"));

    let path = fs::canonicalize(&envfile)
        .with_context(|| format!("err load envfile ({})", envfile))
        .unwrap();

    dotenv::from_path(path).ok();

    let debug = match env::var("DEBUG") {
        Err(_) => false,
        Ok(v) => v.parse::<bool>().unwrap_or_default(),
    };

    (logger::init(debug), db::init(debug).await)
}
