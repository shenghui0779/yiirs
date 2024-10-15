use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};
use http::HeaderName;
use nanoid::nanoid;

pub async fn handle(mut request: Request, next: Next) -> Response {
    let req_id = HeaderValue::from_str(&nanoid!(32)).unwrap_or(HeaderValue::from_static("unknown"));
    request
        .headers_mut()
        .insert(HeaderName::from_static("x-request-id"), req_id.to_owned());

    let mut response = next.run(request).await;
    response
        .headers_mut()
        .insert(HeaderName::from_static("x-request-id"), req_id);
    response
}
