mod config;
mod crypto;
mod entity;
mod middleware;
mod result;
mod router;
mod service;
mod util;

use std::net::SocketAddr;

use util::AppState;

#[tokio::main]
async fn main() {
    let cfg = config::init().await;

    let _guard = config::logger::init(&cfg);
    let db = config::db::init(&cfg).await;
    let redis = config::redis::init(&cfg);

    serve(AppState { cfg, db, redis }).await;
}

async fn serve(state: AppState) {
    // run it with hyper on localhost:8000
    let addr = SocketAddr::from((
        [0, 0, 0, 0],
        state.cfg.get_int("app.port").unwrap_or(8000) as u16,
    ));

    let listener = tokio::net::TcpListener::bind(format!(
        "0.0.0.0:{}",
        state.cfg.get_int("app.port").unwrap_or(8000)
    ))
    .await
    .unwrap();

    tracing::debug!("listening on {}", addr);

    axum::serve(listener, router::app::init(state))
        .await
        .unwrap();
}
