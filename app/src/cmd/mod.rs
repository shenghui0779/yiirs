use clap::{Parser, Subcommand};

pub mod hello;

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
    Hello {
        #[arg(short, long, default_value = "world")]
        name: String,
    },
    Serve,
}
