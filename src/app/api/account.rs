use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use axum_extra::extract::WithRejection;
use validator::Validate;

use crate::internal::{
    result::{code::Code, rejection::IRejection, ApiResult},
    util::identity::{Identity, Role},
};

use crate::app::service::{
    self,
    account::{ReqCreate, RespInfo, RespList},
};

pub async fn create(
    Extension(identity): Extension<Identity>,
    WithRejection(Json(req), _): IRejection<Json<ReqCreate>>,
) -> ApiResult<()> {
    if !identity.is_role(Role::Super) {
        return Err(Code::ErrPerm(None));
    }
    if let Err(e) = req.validate() {
        return Err(Code::ErrParams(Some(e.to_string())));
    }
    service::account::create(req).await
}

pub async fn info(
    Extension(identity): Extension<Identity>,
    Path(account_id): Path<u64>,
) -> ApiResult<RespInfo> {
    if !identity.is_role(Role::Super) {
        return Err(Code::ErrPerm(None));
    }
    service::account::info(account_id).await
}

pub async fn list(
    Extension(identity): Extension<Identity>,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<RespList> {
    if !identity.is_role(Role::Super) {
        return Err(Code::ErrPerm(None));
    }
    service::account::list(query).await
}
