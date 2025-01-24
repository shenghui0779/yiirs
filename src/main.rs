mod cmd;
mod internal;

use clap::Parser;

fn main() {
    // 解析command
    let cli = cmd::Cli::parse();
    // 处理command
    if let Some(v) = cli.command {
        match v {
            cmd::Command::New { name, axum, app } => cmd::project::run(name, axum, app),
            cmd::Command::App { name, axum } => cmd::app::run(name, axum),
        }
    }
}
