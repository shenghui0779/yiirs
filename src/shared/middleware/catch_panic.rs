use std::panic::AssertUnwindSafe;

use futures::FutureExt;

use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::shared::result::code::Code;

pub async fn handle(request: Request, next: Next) -> Response {
    if let Ok(resp) = AssertUnwindSafe(next.run(request)).catch_unwind().await {
        return resp;
    }
    Code::ErrSystem(None).into_response()
}
