use axum::{Extension, Json};
use axum_extra::extract::WithRejection;
use validator::Validate;

use crate::result::{
    rejection::IRejection,
    response::{ApiErr, ApiOK, Result},
};
use crate::service::auth::{ReqLogin, RespLogin};
use crate::service::identity::Identity;

pub async fn login(
    WithRejection(Json(req), _): IRejection<Json<ReqLogin>>,
) -> Result<ApiOK<RespLogin>> {
    if let Err(err) = req.validate() {
        return Err(ApiErr::ErrParams(Some(err.to_string())));
    }

    crate::service::auth::login(req).await
}

pub async fn logout(Extension(identity): Extension<Identity>) -> Result<ApiOK<()>> {
    if identity.id() == 0 {
        return Ok(ApiOK(None));
    }

    crate::service::auth::logout(identity).await
}
