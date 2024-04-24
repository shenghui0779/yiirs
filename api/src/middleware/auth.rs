use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::result::response::ApiErr;
use crate::service::identity::Identity;

pub async fn handle(request: Request, next: Next) -> Response {
    let identity = request.extensions().get::<Identity>();

    match identity {
        None => return ApiErr::ErrAuth(None).into_response(),
        Some(v) => match v.check().await {
            Ok(_) => (),
            Err(err) => return ApiErr::ErrAuth(Some(err.to_string())).into_response(),
        },
    }

    next.run(request).await
}
