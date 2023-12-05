pub mod db;
pub mod logger;
pub mod redis;

use config::Config;
use std::fs;

use crate::util::AppState;

pub fn init(cfg_file: &String) -> Config {
    let path = fs::canonicalize(cfg_file)
        .unwrap_or_else(|e| panic!("配置文件加载失败：{} - {}", e, cfg_file));

    Config::builder()
        .add_source(config::File::with_name(path.to_str().unwrap()))
        .build()
        .unwrap_or_else(|e| panic!("配置文件加载失败：{}", e))
}

pub async fn app_state(cfg: Config) -> AppState {
    let db = db::init(&cfg).await;
    let redis = redis::init(&cfg);

    AppState { cfg, db, redis }
}
