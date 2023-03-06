mod config;
mod entity;
mod middlewares;
mod result;
mod router;
mod service;
mod util;

#[tokio::main]
async fn main() {
    let _guard = config::init().await;

    serve().await;
}

async fn serve() {
    // run it with hyper on localhost:8000
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(router::app::init().into_make_service())
        .await
        .unwrap();
}
