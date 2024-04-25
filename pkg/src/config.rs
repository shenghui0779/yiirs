use config::Config;
use std::{fs, sync::OnceLock};

static CFG: OnceLock<Config> = OnceLock::new();

pub fn init(cfg_file: &str) {
    let path = fs::canonicalize(cfg_file)
        .unwrap_or_else(|e| panic!("配置文件加载失败：{} - {}", e, cfg_file));

    let cfg = Config::builder()
        .add_source(config::File::with_name(path.to_str().unwrap()))
        .build()
        .unwrap_or_else(|e| panic!("配置文件加载失败：{}", e));

    let _ = CFG.set(cfg);
}

pub fn global() -> &'static Config {
    CFG.get().unwrap_or_else(|| panic!("配置文件未初始化"))
}
