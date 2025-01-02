use salvo::{handler, Request};
use validator::Validate;

use crate::internal::{
    result::{code::Code, reply, ApiResult},
    util::identity::Identity,
};

use crate::app::service::{
    self,
    auth::{ReqLogin, RespLogin},
};

#[handler]
pub async fn login(req: &mut Request) -> ApiResult<RespLogin> {
    let params = req.parse_json::<ReqLogin>().await.map_err(|e| {
        tracing::error!(err = ?e, "req.parse_json");
        Code::ErrParams(Some("参数解析出错".to_string()))
    })?;
    if let Err(e) = params.validate() {
        return Err(Code::ErrParams(Some(e.to_string())));
    }
    service::auth::login(params).await
}

#[handler]
pub async fn logout(req: &mut Request) -> ApiResult<()> {
    let empty = Identity::empty();
    let id = req.extensions().get::<Identity>().unwrap_or(&empty);
    if id.id() == 0 {
        return Ok(reply::OK(None));
    }
    service::auth::logout(id).await
}
