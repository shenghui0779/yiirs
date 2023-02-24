use axum::{http::Request, middleware::Next, response::Response};

use crate::util::auth::Identity;

pub async fn handle<B>(mut request: Request<B>, next: Next<B>) -> Response {
    let identity = match request.headers().get("authorization") {
        None => Identity::empty(),
        Some(v) => match v.to_str() {
            Err(err) => {
                tracing::error!(error = ?err, "err get header(authorization)");
                Identity::empty()
            }
            Ok(v) => Identity::from_auth_token(v.to_string()),
        },
    };

    request.extensions_mut().insert(identity);

    next.run(request).await
}
