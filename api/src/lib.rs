use config::Config;
use redis::Client as RedisClient;
use sea_orm::DatabaseConnection;

pub mod auth;
pub mod controller;
pub mod middleware;
pub mod router;

#[derive(Debug, Clone)]
pub struct AppState {
    pub cfg: Config,
    pub db: DatabaseConnection,
    pub redis: RedisClient,
}

async fn app_state(cfg: Config) -> AppState {
    let db = library::core::db::init(&cfg).await;
    let redis = library::core::redis::init(&cfg);

    AppState { cfg, db, redis }
}

pub async fn serve(cfg: Config) {
    // run it with hyper on localhost:8000
    let addr = cfg.get_int("app.port").unwrap_or(8000);
    let state = app_state(cfg).await;

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", addr))
        .await
        .unwrap();

    tracing::info!("listening on {}", addr);

    axum::serve(listener, router::app::init(state))
        .await
        .unwrap();
}