pub mod db;
pub mod logger;
pub mod redis;

use config::Config;
use std::fs;
use tracing_appender::non_blocking::WorkerGuard;

use crate::util::AppState;

pub async fn init(cfg_file: &String) -> (AppState, WorkerGuard) {
    let path = fs::canonicalize(cfg_file)
        .unwrap_or_else(|e| panic!("配置文件加载失败：{} - {}", e, cfg_file));

    let cfg = Config::builder()
        .add_source(config::File::with_name(path.to_str().unwrap()))
        .build()
        .unwrap_or_else(|e| panic!("配置文件加载失败：{}", e));

    let _guard = logger::init(&cfg);
    let db = db::init(&cfg).await;
    let redis = redis::init(&cfg);

    (AppState { cfg, db, redis }, _guard)
}
