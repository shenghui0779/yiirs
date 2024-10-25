use std::collections::HashMap;

use http::header::CONTENT_TYPE;
use http_body_util::BodyExt;
use hyper::HeaderMap;
use salvo::{async_trait, writing::Json, Depot, FlowCtrl, Handler, Request, Response};

use crate::shared::{
    result::code::Code,
    util::{identity::Identity, xtime},
};

pub struct Log;

impl Log {
    pub fn new() -> Self {
        Log {}
    }
}

#[async_trait]
impl Handler for Log {
    async fn handle(
        &self,
        req: &mut Request,
        _depot: &mut Depot,
        resp: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        let enter_time = xtime::now(None);
        let req_method = req.method().to_string();
        let req_uri = req.uri().to_string();
        // let req_header = header_to_string(request.headers());
        let id = match req.extensions().get::<Identity>() {
            Some(v) => v.to_string(),
            None => String::from("<none>"),
        };
        // 获取body
        let (body, code) = drain_body(req).await;
        if let Some(v) = code {
            resp.render(Json(v.to_status()));
            ctrl.skip_rest();
            return;
        }
        // 请求时长
        let duration = (xtime::now(None) - enter_time).to_string();
        tracing::info!(
            method = req_method,
            uri = req_uri,
            // headers = req_header,
            identity = id,
            body = body,
            duration = duration,
            "Request info"
        );
    }
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
    match serde_json::to_string(&map) {
        Ok(v) => v,
        Err(_) => String::from("<none>"),
    }
}

async fn drain_body(req: &mut Request) -> (Option<String>, Option<Code>) {
    let ok = match req.header::<String>(CONTENT_TYPE) {
        Some(v) => {
            v.starts_with("application/json") || v.starts_with("application/x-www-form-urlencoded")
        }
        None => false,
    };
    if !ok {
        return (None, None);
    }
    // this wont work if the body is an long running stream
    let bytes = match req.body_mut().collect().await {
        Ok(v) => v.to_bytes(),
        Err(e) => {
            tracing::error!(error = ?e, "Error body.collect");
            return (None, Some(Code::ErrSystem(None)));
        }
    };
    let body = std::str::from_utf8(&bytes).map(|s| s.to_string()).ok();
    (body, None)
    // let response = next
    //     .run(Request::from_parts(parts, Body::from(bytes)))
    //     .await;
    // Ok((response, body))
}
