use config::Config;
use std::fs;

pub fn init(cfg_file: &String) -> Config {
    let path = fs::canonicalize(cfg_file)
        .unwrap_or_else(|e| panic!("配置文件加载失败：{} - {}", e, cfg_file));

    Config::builder()
        .add_source(config::File::with_name(path.to_str().unwrap()))
        .build()
        .unwrap_or_else(|e| panic!("配置文件加载失败：{}", e))
}
