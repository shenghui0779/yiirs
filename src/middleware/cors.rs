use axum::{
    headers::HeaderName,
    http::{HeaderMap, HeaderValue, Method, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

pub async fn handle<B>(request: Request<B>, next: Next<B>) -> Response {
    let mut cors_headers = HeaderMap::new();

    cors_headers.insert(
        HeaderName::from_static("access-control-allow-origin"),
        HeaderValue::from_static("*"),
    );
    cors_headers.insert(
        HeaderName::from_static("access-control-allow-credentials"),
        HeaderValue::from_static("true"),
    );
    cors_headers.insert(
        HeaderName::from_static("access-control-allow-methods"),
        HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"),
    );
    cors_headers.insert(
        HeaderName::from_static("access-control-allow-headers"),
        HeaderValue::from_static("content-type, authorization, withCredentials"),
    );

    if request.method() == Method::OPTIONS {
        return (StatusCode::NO_CONTENT, cors_headers).into_response();
    }

    let response = next.run(request).await;

    (cors_headers, response).into_response()
}
