mod config;
mod entity;
mod middlewares;
mod result;
mod router;
mod service;
mod util;

use std::{env, path::Path};

use config::{db, logger};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    // cargo run -- /data/config/.env
    if args.len() > 1 {
        let envfile = Path::new(&args[1]).canonicalize().unwrap();
        dotenv::from_path(envfile).ok();
    } else {
        dotenv().ok();
    }

    let _guard = logger::init().await;

    db::init().await;

    serve().await;
}

async fn serve() {
    // run it with hyper on localhost:8000
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(router::app::init().into_make_service())
        .await
        .unwrap();
}
