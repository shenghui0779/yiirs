use salvo::{cors::Cors, handler, Router};

use crate::internal;

use super::middleware;

pub mod route;

pub fn init() -> Router {
    // 开放
    let open = Router::new().push(route::login());
    // 需授权
    let auth = Router::new()
        .hoop(middleware::auth::Auth::new())
        .push(route::logout())
        .push(route::account())
        .push(route::project());
    // cors
    let cors = Cors::very_permissive()
        .expose_headers(vec![internal::middleware::trace::TRACE_ID])
        .into_handler();
    // 路由组册
    Router::new()
        .get(root)
        .hoop(cors)
        .hoop(internal::middleware::trace::Trace::new())
        .hoop(internal::middleware::catch_panic::CatchPanic::new())
        .hoop(internal::middleware::log::Log::new())
        .path("v1")
        .push(open)
        .push(auth)
}

#[handler]
async fn root() -> &'static str {
    "☺ welcome to Rust app"
}
