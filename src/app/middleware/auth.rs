use salvo::{async_trait, Depot, FlowCtrl, Handler, Request, Response};

use crate::shared::{
    result::{status, ApiResult},
    util::identity::Identity,
};

use super::auth_check;

pub struct Auth;

impl Auth {
    pub fn new() -> Self {
        Auth {}
    }
}

#[async_trait]
impl Handler for Auth {
    async fn handle(&self, req: &mut Request) -> ApiResult<()> {
        let identity = req.extensions().get::<Identity>();
        match identity {
            None => return Err(status::Err::Auth(None)),
            Some(v) => match auth_check(v).await {
                Ok(_) => Ok(status::OK(None)),
                Err(e) => return Err(status::Err::Auth(Some(e.to_string()))),
            },
        }
    }
}
