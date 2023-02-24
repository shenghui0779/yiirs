use axum::{
    http::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{result::response::ApiErr, util::auth::Identity};

pub async fn handle<B>(request: Request<B>, next: Next<B>) -> Response {
    match request.extensions().get::<Identity>() {
        None => return ApiErr::ErrAuth(None).into_response(),
        Some(identity) => match identity.check().await {
            Err(err) => return ApiErr::ErrAuth(Some(err.to_string())).into_response(),
            Ok(_) => next.run(request).await,
        },
    }
}
