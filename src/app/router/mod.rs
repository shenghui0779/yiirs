use salvo::{handler, Router};

use crate::shared;

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
    // 路由组册
    Router::new()
        .get(root)
        .hoop(shared::middleware::trace::Trace::new())
        .hoop(shared::middleware::log::Log::new())
        .path("v1")
        .push(open)
        .push(auth)
}

#[handler]
async fn root() -> &'static str {
    "☺ welcome to Rust app"
}
