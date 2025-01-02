use std::panic;

use app::cmd;
use clap::Parser;
use internal::core::{cache, config, db, logger};
use tracing_appender::non_blocking::WorkerGuard;

pub mod app;
pub mod internal;

#[tokio::main]
async fn main() {
    let cli = cmd::Cli::parse();
    // _guard 必须在 main 函数中才能使日志生效
    let _guard = init(&cli.config).await;
    // catch panic
    panic::set_hook(Box::new(|info| {
        tracing::error!(error = %info, "panic occurred");
    }));
    // 处理subcommand
    if let Some(v) = cli.command {
        match v {
            cmd::Command::Hello { name } => cmd::hello::exec(name),
            cmd::Command::Serve => app::serve().await,
        }
    }
}

async fn init(cfg_file: &str) -> WorkerGuard {
    // 初始化配置
    config::init(cfg_file);
    // 初始化日志
    let _guard = logger::init(Some(config::global()));
    // 初始化数据库
    db::init(config::global()).await;
    // 初始化Redis
    cache::init_redis(config::global());

    _guard
}
