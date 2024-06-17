use axum::{
    body::Body,
    http::Request,
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;

use crate::api::{
    controller::{account, auth, project},
    middleware,
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
        .layer(axum::middleware::from_fn(middleware::auth::handle));

    Router::new()
        .route("/", get(|| async { "☺ welcome to Rust app" }))
        .nest("/v1", open.merge(auth))
        .layer(axum::middleware::from_fn(pkg::middleware::log::handle))
        .layer(axum::middleware::from_fn(pkg::middleware::identity::handle))
        .layer(axum::middleware::from_fn(pkg::middleware::cors::handle))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                let req_id = match request
                    .headers()
                    .get("x-request-id")
                    .and_then(|value| value.to_str().ok())
                {
                    Some(v) => v.to_string(),
                    None => String::from("unknown"),
                };

                tracing::error_span!("request_id", id = req_id)
            }),
        )
        .layer(axum::middleware::from_fn(pkg::middleware::req_id::handle))
}
