use library::core::{config, logger};

#[tokio::main]
async fn main() {
    let matches = cmd::cli().get_matches();

    // _guard 必须在 main 函数中才能使日志生效
    match matches.subcommand() {
        // Command: serve
        Some((cmd::serve::NAME, sub_matches)) => {
            let cfg = config::init(sub_matches.get_one::<String>(cmd::serve::ARG_FILE).unwrap());
            let _guard = logger::init(Some(&cfg));

            api::serve(cfg).await;
        }
        // Command: hello
        Some((cmd::hello::NAME, _sub_matches)) => println!("hello world"),
        // Unreachable
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
