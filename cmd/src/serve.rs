use clap::{Arg, Command};

pub const NAME: &str = "serve";

pub fn cmd() -> Command {
    Command::new(NAME).about("Run api server").arg(
        Arg::new("FILE")
            .long("config")
            .short('C')
            .help("set config file")
            .required(false)
            .default_value("config.toml"),
    )
}
