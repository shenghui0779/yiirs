use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use axum_extra::extract::WithRejection;
use validator::Validate;

use crate::result::{
    rejection::IRejection,
    response::{ApiErr, ApiOK, Result},
};
use crate::service::{
    account::{ReqCreate, RespInfo, RespList},
    identity::{Identity, Role},
};

pub async fn create(
    Extension(identity): Extension<Identity>,
    WithRejection(Json(req), _): IRejection<Json<ReqCreate>>,
) -> Result<ApiOK<()>> {
    if !identity.is_role(Role::Super) {
        return Err(ApiErr::ErrPerm(None));
    }

    if let Err(err) = req.validate() {
        return Err(ApiErr::ErrParams(Some(err.to_string())));
    }

    crate::service::account::create(req).await
}

pub async fn info(
    Extension(identity): Extension<Identity>,
    Path(account_id): Path<u64>,
) -> Result<ApiOK<RespInfo>> {
    if !identity.is_role(Role::Super) {
        return Err(ApiErr::ErrPerm(None));
    }

    crate::service::account::info(account_id).await
}

pub async fn list(
    Extension(identity): Extension<Identity>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<ApiOK<RespList>> {
    if !identity.is_role(Role::Super) {
        return Err(ApiErr::ErrPerm(None));
    }

    crate::service::account::list(query).await
}
