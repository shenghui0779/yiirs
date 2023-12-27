use clap::{Arg, Command};

pub fn cli() -> Command {
    Command::new("api-tpl-rs")
        .about("rust api service - build with tokio | clap | axum | sea-orm | tracing")
        .version("1.1.0")
        .subcommand_required(false)
        .arg_required_else_help(true)
        .author("shenghui0779")
        .subcommand(
            Command::new("serve").about("Run app server").arg(
                Arg::new("FILE")
                    .long("config")
                    .short('C')
                    .help("set config file")
                    .required(false)
                    .default_value("config.toml"),
            ),
        )
        .subcommand(Command::new("hello").about("Example subcommand"))
}
