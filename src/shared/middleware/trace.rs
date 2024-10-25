use http::{header::AUTHORIZATION, HeaderName, HeaderValue};
use nanoid::nanoid;
use salvo::{async_trait, Depot, FlowCtrl, Handler, Request, Response};

use crate::shared::{crypto::hash, util::identity::Identity};

pub const TRACE_ID_KEY: &str = "x-trace-id";

pub struct Trace;

impl Trace {
    pub fn new() -> Self {
        Trace {}
    }
}

#[async_trait]
impl Handler for Trace {
    async fn handle(
        &self,
        req: &mut Request,
        _depot: &mut Depot,
        resp: &mut Response,
        _ctrl: &mut FlowCtrl,
    ) {
        let hostname = hostname::get()
            .unwrap_or_default()
            .into_string()
            .unwrap_or_default();
        // traceId
        let trace_id = match req.header::<String>(TRACE_ID_KEY) {
            Some(v) => {
                if v.len() != 0 {
                    v
                } else {
                    gen_trace_id(req, &hostname)
                }
            }
            None => gen_trace_id(req, &hostname),
        };
        // Identity
        let token = req.header::<String>(AUTHORIZATION);
        let id = match token {
            None => Identity::empty(),
            Some(v) => Identity::from_auth_token(v),
        };
        // 设置 trace span
        let span = tracing::info_span!("trace", hostname, trace_id, identity = id.to_string());
        // 进入span，生命周期内所有事件都关联到该span
        let _enter = span.enter();
        // 设置 Identity
        req.extensions_mut().insert(id);
        // 设置返回header
        resp.headers_mut().insert(
            HeaderName::from_static(TRACE_ID_KEY),
            HeaderValue::from_str(&trace_id).unwrap_or(HeaderValue::from_static("")),
        );
    }
}

fn gen_trace_id(req: &mut Request, hostname: &str) -> String {
    let id = hash::md5(format!("{}/{}", hostname, nanoid!(32)).as_bytes());
    req.headers_mut().insert(
        HeaderName::from_static(TRACE_ID_KEY),
        HeaderValue::from_str(&id).unwrap_or(HeaderValue::from_static("")),
    );
    id
}
