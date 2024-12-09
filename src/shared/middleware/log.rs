use std::collections::HashMap;

use axum::{
    body::Body,
    extract::Request,
    http::header::CONTENT_TYPE,
    middleware::Next,
    response::{IntoResponse, Response},
};
use http_body_util::BodyExt;
use hyper::HeaderMap;

use crate::shared::{result::code::Code, util::xtime};

pub async fn handle(request: Request, next: Next) -> Response {
    let enter_time = xtime::now(None);
    let req_method = request.method().to_string();
    let req_uri = request.uri().to_string();
    // let req_header = header_to_string(request.headers());
    // 获取body
    let (response, body) = match drain_body(request, next).await {
        Err(e) => return e.into_response(),
        Ok(v) => v,
    };
    // 请求时长
    let duration = (xtime::now(None) - enter_time).to_string();
    tracing::info!(
        method = req_method,
        uri = req_uri,
        // headers = req_header,
        body = body,
        duration = duration,
        "Request info"
    );
    response
}

#[allow(dead_code)]
fn header_to_string(h: &HeaderMap) -> String {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for k in h.keys() {
        let mut vals: Vec<String> = Vec::new();
        for v in h.get_all(k) {
            if let Ok(s) = v.to_str() {
                vals.push(s.to_string())
            }
        }
        map.insert(k.to_string(), vals);
    }
    serde_json::to_string(&map).unwrap_or_else(|_| String::from("<none>"))
}

async fn drain_body(request: Request, next: Next) -> Result<(Response, Option<String>), Code> {
    let ok = match request
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
    {
        Some(v) => {
            v.starts_with("application/json") || v.starts_with("application/x-www-form-urlencoded")
        }
        None => false,
    };
    if !ok {
        return Ok((next.run(request).await, None));
    }

    let (parts, body) = request.into_parts();
    // this wont work if the body is a long running stream
    let bytes = match body.collect().await {
        Ok(v) => v.to_bytes(),
        Err(e) => {
            tracing::error!(err = ?e, "Error body.collect");
            return Err(Code::ErrSystem(None));
        }
    };
    let body = std::str::from_utf8(&bytes).map(|s| s.to_string()).ok();
    let response = next
        .run(Request::from_parts(parts, Body::from(bytes)))
        .await;
    Ok((response, body))
}
