use salvo::{handler, Router};

use crate::{
    app::{self, api},
    shared,
};

pub fn init() -> Router {
    // 开放
    let open = Router::with_path("login").post(api::auth::login);
    // 需授权
    let auth = Router::new()
        .hoop(app::middleware::auth::Auth::new())
        .push(Router::with_path("logout").get(api::auth::logout))
        .push(
            Router::with_path("accounts")
                .get(api::account::list)
                .post(api::account::create)
                .push(Router::with_path("<account_id>").get(api::account::info)),
        )
        .push(
            Router::with_path("projects")
                .get(api::project::list)
                .post(api::project::create)
                .push(Router::with_path("<project_id>").get(api::project::detail)),
        );
    // 路由组册
    Router::new()
        .get(root)
        .path("v1")
        .hoop(shared::middleware::trace::Trace::new())
        .hoop(shared::middleware::log::Log::new())
        .push(open)
        .push(auth)
}

#[handler]
async fn root() -> &'static str {
    "☺ welcome to Rust app"
}
