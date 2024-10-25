use salvo::prelude::*;

use crate::shared::core::config;

pub mod api;
pub mod cmd;
pub mod middleware;
pub mod model;
pub mod router;
pub mod service;

pub async fn serve() {
    let addr = config::global().get_int("app.port").unwrap_or(8000);
    let acceptor = TcpListener::new(format!("0.0.0.0:{}", addr)).bind().await;
    let router = router::route::init();
    tracing::info!("listening on {}", addr);
    Server::new(acceptor).serve(router).await;
}
