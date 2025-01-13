use salvo::{cors::Cors, handler, Router};

use crate::internal;

use super::middleware;

pub mod route;

pub fn init() -> Router {
    // cors
    let cors = Cors::very_permissive()
        .expose_headers(vec![internal::middleware::trace::TRACE_ID])
        .into_handler();
    // 路由组册
    Router::new()
        .get(root)
        .hoop(cors)
        .hoop(internal::middleware::trace::Trace)
        .hoop(internal::middleware::catch_panic::CatchPanic)
        .hoop(internal::middleware::log::Log)
        .push(v1())
}

#[handler]
async fn root() -> &'static str {
    "☺ welcome to Rust app"
}

pub fn v1() -> Router {
    // 开放
    let open = Router::new().push(route::login());
    // 需授权
    let auth = Router::new()
        .hoop(middleware::auth::Auth)
        .push(route::logout())
        .push(route::account())
        .push(route::project());
    // v1
    Router::with_path("v1").push(open).push(auth)
}
