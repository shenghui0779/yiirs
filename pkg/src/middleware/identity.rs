use axum::{extract::Request, middleware::Next, response::Response};
use http::header::AUTHORIZATION;

use crate::identity::Identity;

pub async fn handle(mut request: Request, next: Next) -> Response {
    let token = request.headers().get(AUTHORIZATION);
    let identity = match token {
        None => Identity::empty(),
        Some(v) => match v.to_str() {
            Ok(v) => Identity::from_auth_token(v.to_string()),
            Err(e) => {
                tracing::error!(error = ?e, "error get header(authorization)");
                Identity::empty()
            }
        },
    };
    request.extensions_mut().insert(identity);
    next.run(request).await
}
