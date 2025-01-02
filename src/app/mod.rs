use crate::internal::core::config;

pub mod api;
pub mod cmd;
pub mod middleware;
pub mod model;
pub mod router;
pub mod service;

pub async fn serve() {
    // run it with hyper on localhost:8000
    let addr = config::global().get_int("app.port").unwrap_or(8000);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", addr))
        .await
        .unwrap();

    tracing::info!("listening on {}", addr);

    axum::serve(listener, router::route::init()).await.unwrap();
}
