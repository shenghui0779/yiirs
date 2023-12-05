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

    // _guard 必须在 main 函数中才能使日志生效
    match matches.subcommand() {
        Some(("serve", sub_matches)) => {
            let cfg = config::init(sub_matches.get_one::<String>("FILE").unwrap());
            let _guard = config::logger::init(Some(&cfg));

            cmd::server::serve(config::app_state(cfg).await).await;
        }
        Some(("hello", _sub_matches)) => println!("hello world"),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
