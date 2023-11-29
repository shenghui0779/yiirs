use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use http::header::AUTHORIZATION;

use crate::util::{auth::Identity, AppState};

pub async fn handle(State(state): State<AppState>, mut request: Request, next: Next) -> Response {
    let token = request.headers().get(AUTHORIZATION);

    let identity = match token {
        None => Identity::empty(),
        Some(v) => match v.to_str() {
            Ok(v) => Identity::from_auth_token(&state.cfg, v.to_string()),
            Err(err) => {
                tracing::error!(error = ?err, "err get header(authorization)");
                Identity::empty()
            }
        },
    };

    request.extensions_mut().insert(identity);

    next.run(request).await
}
