mod cmd;
mod internal;

use clap::Parser;

fn main() {
    // 解析command
    let cli = cmd::Cli::parse();
    // 处理command
    if let Some(v) = cli.command {
        match v {
            cmd::Command::New { name, axum, apps } => cmd::project::run(name, axum, apps),
            cmd::Command::App { name, axum } => cmd::app::run(name, axum),
        }
    }
}
