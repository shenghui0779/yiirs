use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::internal::{result::code::Code, util::identity::Identity};

use super::auth_check;

pub async fn handle(request: Request, next: Next) -> Response {
    let identity = request.extensions().get::<Identity>();
    match identity {
        None => return Code::ErrAuth(None).into_response(),
        Some(v) => match auth_check(v).await {
            Ok(_) => (),
            Err(e) => return Code::ErrAuth(Some(e.to_string())).into_response(),
        },
    }
    next.run(request).await
}
