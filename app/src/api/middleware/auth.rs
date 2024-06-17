use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};

use pkg::identity::Identity;
use pkg::result::response::ApiErr;

use crate::api::auth_check;

pub async fn handle(request: Request, next: Next) -> Response {
    let identity = request.extensions().get::<Identity>();
    match identity {
        None => return ApiErr::ErrAuth(None).into_response(),
        Some(v) => match auth_check(v).await {
            Ok(_) => (),
            Err(e) => return ApiErr::ErrAuth(Some(e.to_string())).into_response(),
        },
    }
    next.run(request).await
}
