use axum::{
    extract::Request,
    http::{HeaderMap, HeaderValue, Method, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use http::header::{
    ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
    ACCESS_CONTROL_ALLOW_ORIGIN, ACCESS_CONTROL_EXPOSE_HEADERS,
};

pub async fn handle(request: Request, next: Next) -> Response {
    let mut cors_headers = HeaderMap::new();

    cors_headers.insert(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
    cors_headers.insert(
        ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HeaderValue::from_static("true"),
    );
    cors_headers.insert(
        ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"),
    );
    cors_headers.insert(
        ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("content-type, authorization, withCredentials"),
    );
    // cors_headers.insert(
    //     ACCESS_CONTROL_EXPOSE_HEADERS,
    //     HeaderValue::from_static("服务器暴露一些自定义的头信息，允许客户端访问"),
    // );

    if request.method() == Method::OPTIONS {
        return (StatusCode::NO_CONTENT, cors_headers).into_response();
    }

    let response = next.run(request).await;

    (cors_headers, response).into_response()
}
