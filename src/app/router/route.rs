use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;

use crate::{
    app::{
        self,
        api::{account, auth, project},
    },
    shared::middleware,
};

pub fn init() -> Router {
    // 开放
    let open = Router::new().route("/login", post(auth::login));

    // 需授权
    let auth = Router::new()
        .route("/logout", get(auth::logout))
        .route("/accounts", get(account::list).post(account::create))
        .route("/accounts/:account_id", get(account::info))
        .route("/projects", get(project::list).post(project::create))
        .route("/projects/:project_id", get(project::detail))
        .layer(axum::middleware::from_fn(app::middleware::auth::handle));

    // 路由组册
    Router::new()
        .route("/", get(|| async { "☺ welcome to Rust app" }))
        .nest("/v1", open.merge(auth))
        .layer(axum::middleware::from_fn(middleware::log::handle))
        .layer(CorsLayer::very_permissive().expose_headers(vec![middleware::trace::TRACE_ID]))
        .layer(axum::middleware::from_fn(middleware::catch_panic::handle))
        .layer(axum::middleware::from_fn(middleware::trace::handle))
}
