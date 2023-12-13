mod cmd;
mod config;
mod crypto;
mod db;
mod logger;
mod middleware;
mod redis;
mod result;
mod router;
mod service;
mod util;

#[tokio::main]
async fn main() {
    let matches = cmd::cli().get_matches();

    // _guard 必须在 main 函数中才能使日志生效
    match matches.subcommand() {
        // Command: serve
        Some(("serve", sub_matches)) => {
            let cfg = config::init(sub_matches.get_one::<String>("FILE").unwrap());
            let _guard = logger::init(Some(&cfg));
            cmd::server::serve(util::app_state(cfg).await).await;
        }
        // Command: hello
        Some(("hello", _sub_matches)) => println!("hello world"),
        // Unreachable
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
