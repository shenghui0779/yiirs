use std::net::SocketAddr;

use crate::{router, util::AppState};

pub async fn serve(state: AppState) {
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

    tracing::info!("listening on {}", addr);

    axum::serve(listener, router::app::init(state))
        .await
        .unwrap();
}
