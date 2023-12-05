mod cmd;
mod config;
mod crypto;
mod entity;
mod middleware;
mod result;
mod router;
mod service;
mod util;

#[tokio::main]
async fn main() {
    let matches = cmd::cli().get_matches();

    match matches.subcommand() {
        Some(("serve", sub_matches)) => {
            let (state, _guard) =
                config::init(sub_matches.get_one::<String>("FILE").unwrap()).await;
            cmd::server::serve(state).await;
        }
        Some(("hello", _sub_matches)) => println!("hello world"),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
