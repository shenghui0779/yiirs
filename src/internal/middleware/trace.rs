use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};
use http::{header::AUTHORIZATION, HeaderName};
use nanoid::nanoid;
use tracing::Instrument;

use crate::internal::{crypto::hash, util::identity::Identity};

pub const TRACE_ID: HeaderName = HeaderName::from_static("x-trace-id");

pub async fn handle(mut request: Request, next: Next) -> Response {
    let hostname = hostname::get()
        .unwrap_or_default()
        .into_string()
        .unwrap_or_default();
    // traceId
    let trace_id = match request
        .headers()
        .get(TRACE_ID)
        .and_then(|v| v.to_str().ok())
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
    // Identity
    let token = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok());
    let id = match token {
        None => Identity::empty(),
        Some(v) => Identity::from_auth_token(v.to_string()),
    };
    let id_str = id.to_string();
    // 设置 Identity
    request.extensions_mut().insert(id);
    // 设置 trace span
    let span = tracing::info_span!("trace", hostname, trace_id, identity = id_str);
    let mut response = next.run(request).instrument(span).await;
    // 设置返回header
    response.headers_mut().insert(
        TRACE_ID,
        HeaderValue::from_str(&trace_id).unwrap_or(HeaderValue::from_static("")),
    );
    response
}

fn gen_trace_id(req: &mut Request, hostname: &str) -> String {
    let id = hash::md5(format!("{}/{}", hostname, nanoid!(32)).as_bytes());
    req.headers_mut().insert(
        TRACE_ID,
        HeaderValue::from_str(&id).unwrap_or(HeaderValue::from_static("")),
    );
    id
}
