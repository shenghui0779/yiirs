use clap::Command;

pub const NAME: &str = "hello";

pub fn cmd() -> Command {
    Command::new(NAME).about("Example subcommand")
}
