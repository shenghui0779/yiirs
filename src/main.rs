mod axum;
mod cmd;
mod salvo;

use std::{env, io};

use clap::Parser;
use tera::Context;

fn main() {
    // 获取当前目录
    let dir = env::current_dir().unwrap().canonicalize().unwrap();

    // 解析command
    let cli = cmd::Cli::parse();
    // 处理command
    if let Some(v) = cli.command {
        match v {
            cmd::Command::New { name, axum } => {
                let root = dir.join(&name);
                // 判断目录是否为空
                let is_empty = match root.read_dir() {
                    Ok(entries) => entries.count() == 0,
                    Err(e) => match e.kind() {
                        io::ErrorKind::NotFound => true,
                        _ => panic!("{}", e),
                    },
                };
                if !is_empty {
                    println!("👿 目录({:?})不为空，请确认！", root);
                    return;
                }
                // 创建文件
                let mut ctx = Context::new();
                ctx.insert("name", &name);
                let tera = if axum { axum::new() } else { salvo::new() };
                cmd::build(root, tera, ctx);
                println!("🍺 项目创建完成！请阅读README")
            }
        }
    }
}
