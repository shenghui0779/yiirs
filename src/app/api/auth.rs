use axum::{Extension, Json};
use axum_extra::extract::WithRejection;
use validator::Validate;

use crate::shared::{
    result::{code::Code, rejection::IRejection, reply, ApiResult},
    util::identity::Identity,
};

use crate::app::service::{
    self,
    auth::{ReqLogin, RespLogin},
};

pub async fn login(
    WithRejection(Json(req), _): IRejection<Json<ReqLogin>>,
) -> ApiResult<RespLogin> {
    if let Err(e) = req.validate() {
        return Err(Code::ErrParams(Some(e.to_string())));
    }
    service::auth::login(req).await
}

pub async fn logout(Extension(identity): Extension<Identity>) -> ApiResult<()> {
    if identity.id() == 0 {
        return Ok(reply::OK(None));
    }
    service::auth::logout(identity).await
}
