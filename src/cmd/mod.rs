pub mod app;
pub mod project;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    New {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        axum: bool,
        #[arg(short, long)]
        apps: Vec<String>,
    },
    App {
        #[arg(short, long)]
        name: Vec<String>,
        #[arg(short, long)]
        axum: bool,
    },
}
