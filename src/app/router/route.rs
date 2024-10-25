use salvo::Router;

use crate::app::api;

pub fn login() -> Router {
    Router::with_path("login").post(api::auth::login)
}

pub fn logout() -> Router {
    Router::with_path("logout").get(api::auth::logout)
}

pub fn account() -> Router {
    Router::with_path("accounts")
        .get(api::account::list)
        .post(api::account::create)
        .push(Router::with_path("<account_id>").get(api::account::info))
}

pub fn project() -> Router {
    Router::with_path("projects")
        .get(api::project::list)
        .post(api::project::create)
        .push(Router::with_path("<project_id>").get(api::project::detail))
}
