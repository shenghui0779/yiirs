use clap::Command;

pub mod hello;
pub mod serve;

pub fn cli() -> Command {
    Command::new("api-tpl-rs")
        .about("rust api service - build with tokio | clap | axum | sea-orm | tracing")
        .version("1.1.0")
        .subcommand_required(false)
        .arg_required_else_help(true)
        .author("shenghui0779")
        .subcommand(serve::cmd())
        .subcommand(hello::cmd())
}
