use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use clap::{Parser, Subcommand};
use tera::Tera;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE", default_value = "config.toml")]
    pub config: String,
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    New {
        #[arg(short, long, default_value = "world")]
        name: String,
        #[arg(short, long, default_value = "false")]
        axum: bool,
    },
}

pub fn build(root: PathBuf, tera: Tera, ctx: tera::Context) {
    // 渲染并打印每个模板
    for filename in tera.get_template_names() {
        let content = tera.render(filename, &ctx).unwrap();
        let path = root.join(filename);
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir).unwrap();
        }
        // 创建文件
        let mut file = File::create(path).unwrap();
        // 将内容写入文件
        file.write_all(content.as_bytes()).unwrap();
        println!("{}", filename)
    }
}
