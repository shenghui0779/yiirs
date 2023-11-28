pub mod db;
pub mod logger;
pub mod redis;

use config::Config;
use std::{env, fs};

pub async fn init() -> Config {
    // cargo run -- /data/config/config.yaml
    let cfg_file = env::args().nth(1).unwrap_or(String::from("config.yaml"));
    let path = fs::canonicalize(&cfg_file).unwrap_or_else(|e| panic!("配置文件加载失败：{}", e));

    Config::builder()
        .add_source(config::File::with_name(path.to_str().unwrap()))
        .build()
        .unwrap_or_else(|e| panic!("配置文件加载失败：{}", e))
}
