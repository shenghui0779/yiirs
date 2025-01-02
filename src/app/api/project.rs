use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use axum_extra::extract::WithRejection;
use validator::Validate;

use crate::internal::{
    result::{code::Code, rejection::IRejection, ApiResult},
    util::identity::Identity,
};

use crate::app::service::{
    self,
    project::{ReqCreate, RespDetail, RespList},
};

pub async fn create(
    Extension(id): Extension<Identity>,
    WithRejection(Json(req), _): IRejection<Json<ReqCreate>>,
) -> ApiResult<()> {
    if let Err(e) = req.validate() {
        return Err(Code::ErrParams(Some(e.to_string())));
    }
    service::project::create(id, req).await
}

pub async fn list(
    Extension(id): Extension<Identity>,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<RespList> {
    service::project::list(id, query).await
}

pub async fn detail(
    Extension(id): Extension<Identity>,
    Path(project_id): Path<u64>,
) -> ApiResult<RespDetail> {
    service::project::detail(id, project_id).await
}
