use std::collections::HashMap;

use axum::{
    body::Body,
    http::{header::CONTENT_TYPE, Request},
    middleware::Next,
    response::{IntoResponse, Response},
};
use hyper::HeaderMap;

use crate::{result::response::ApiErr, util::auth::Identity};

pub async fn handle<B>(request: Request<Body>, next: Next<Body>) -> Response {
    let enter_time = chrono::Local::now();
    let req_method = request.method().to_string();
    let req_uri = request.uri().to_string();
    let req_header = header_to_string(request.headers());
    let identity = match request.extensions().get::<Identity>() {
        Some(v) => v.to_string(),
        None => String::from("<none>"),
    };

    let (response, body) = match drain_body(request, next).await {
        Err(err) => return err.into_response(),
        Ok(v) => v,
    };

    let duration = chrono::Local::now()
        .signed_duration_since(enter_time)
        .to_string();

    tracing::info!(
        method = req_method,
        uri = req_uri,
        headers = req_header,
        identity = identity,
        body = body,
        duration = duration,
        "请求记录"
    );

    response
}

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

    match serde_json::to_string(&map) {
        Ok(v) => v,
        Err(_) => String::from("<nil>"),
    }
}

async fn drain_body(
    request: Request<Body>,
    next: Next<Body>,
) -> Result<(Response, Option<String>), ApiErr> {
    let ok = match request
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
    {
        Some(v) => {
            if v.starts_with("application/json")
                || v.starts_with("application/x-www-form-urlencoded")
            {
                true
            } else {
                false
            }
        }
        None => false,
    };

    if !ok {
        return Ok((next.run(request).await, None));
    }

    let (parts, body) = request.into_parts();

    // this wont work if the body is an long running stream
    let bytes = match hyper::body::to_bytes(body).await {
        Ok(b) => b,
        Err(err) => {
            tracing::error!(error = ?err, "err parse request body");
            return Err(ApiErr::ErrSystem(None));
        }
    };

    let body = std::str::from_utf8(&bytes)
        .and_then(|s| Ok(s.to_string()))
        .ok();

    let response = next
        .run(Request::from_parts(parts, Body::from(bytes)))
        .await;

    Ok((response, body))
}
