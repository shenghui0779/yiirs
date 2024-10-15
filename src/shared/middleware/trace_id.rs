use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};
use http::HeaderName;
use nanoid::nanoid;

use crate::shared::crypto::hash;

pub const TRACE_KEY: &str = "x-trace-id";

pub async fn handle(mut request: Request, next: Next) -> Response {
    let hostname = hostname::get()
        .unwrap_or_default()
        .into_string()
        .unwrap_or_default();
    let trace_id = match request
        .headers()
        .get(TRACE_KEY)
        .and_then(|value| value.to_str().ok())
    {
        Some(v) => {
            if v.len() != 0 {
                v.to_string()
            } else {
                gen_trace_id(&mut request, &hostname)
            }
        }
        None => gen_trace_id(&mut request, &hostname),
    };
    // 设置 trace span
    let span = tracing::info_span!("trace", hostname, trace_id);
    // 进入span，生命周期内所有事件都关联到该span
    let _enter = span.enter();
    // 设置返回header
    let mut response = next.run(request).await;
    response.headers_mut().insert(
        HeaderName::from_static(TRACE_KEY),
        HeaderValue::from_str(&trace_id).unwrap_or(HeaderValue::from_static("")),
    );
    response
}

fn gen_trace_id(req: &mut Request, hostname: &str) -> String {
    let id = hash::md5(format!("{}/{}", hostname, nanoid!(32)).as_bytes());
    req.headers_mut().insert(
        HeaderName::from_static(TRACE_KEY),
        HeaderValue::from_str(&id).unwrap_or(HeaderValue::from_static("")),
    );
    id
}
