use axum::{Extension, Json};
use axum_extra::extract::WithRejection;
use validator::Validate;

use crate::shared::{
    result::{
        rejection::IRejection,
        response::{ApiErr, ApiOK},
        Result,
    },
    util::identity::Identity,
};

use crate::app::service::{
    self,
    auth::{ReqLogin, RespLogin},
};

pub async fn login(
    WithRejection(Json(req), _): IRejection<Json<ReqLogin>>,
) -> Result<ApiOK<RespLogin>> {
    if let Err(e) = req.validate() {
        return Err(ApiErr::ErrParams(Some(e.to_string())));
    }
    service::auth::login(req).await
}

pub async fn logout(Extension(identity): Extension<Identity>) -> Result<ApiOK<()>> {
    if identity.id() == 0 {
        return Ok(ApiOK(None));
    }
    service::auth::logout(identity).await
}
