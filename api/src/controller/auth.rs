use axum::{Extension, Json};
use axum_extra::extract::WithRejection;
use service::auth::{ReqLogin, RespLogin};
use service::identity::Identity;
use validator::Validate;

use library::result::{
    rejection::IRejection,
    response::{ApiErr, ApiOK, Result},
};

pub async fn login(
    WithRejection(Json(req), _): IRejection<Json<ReqLogin>>,
) -> Result<ApiOK<RespLogin>> {
    if let Err(err) = req.validate() {
        return Err(ApiErr::ErrParams(Some(err.to_string())));
    }

    service::auth::login(req).await
}

pub async fn logout(Extension(identity): Extension<Identity>) -> Result<ApiOK<()>> {
    if identity.id() == 0 {
        return Ok(ApiOK(None));
    }

    service::auth::logout(identity).await
}
