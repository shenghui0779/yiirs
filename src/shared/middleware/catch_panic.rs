use std::panic::AssertUnwindSafe;

use futures::FutureExt;
use salvo::{async_trait, writing::Json, Depot, FlowCtrl, Handler, Request, Response};

use crate::shared::result::code::Code;

pub struct CatchPanic;

impl CatchPanic {
    #[inline]
    pub fn new() -> Self {
        CatchPanic {}
    }
}

#[async_trait]
impl Handler for CatchPanic {
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        resp: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        if let Err(_) = AssertUnwindSafe(ctrl.call_next(req, depot, resp))
            .catch_unwind()
            .await
        {
            resp.render(Json(Code::ErrSystem(None).to_reply()));
        }
    }
}
