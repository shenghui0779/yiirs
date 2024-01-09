use library::core::cfg;

pub mod auth;
pub mod controller;
pub mod middleware;
pub mod router;

pub async fn serve() {
    // run it with hyper on localhost:8000
    let addr = cfg::config().get_int("app.port").unwrap_or(8000);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", addr))
        .await
        .unwrap();

    tracing::info!("listening on {}", addr);

    axum::serve(listener, router::app::init()).await.unwrap();
}
