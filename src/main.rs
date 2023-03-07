mod config;
mod entity;
mod middlewares;
mod result;
mod router;
mod service;
mod util;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let _guard = config::init().await;

    serve().await;
}

async fn serve() {
    // run it with hyper on localhost:8000
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(router::app::init().into_make_service())
        .await
        .unwrap();
}
