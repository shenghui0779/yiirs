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
    identity::Identity,
    project::{ReqCreate, RespDetail, RespList},
};

pub async fn create(
    Extension(identity): Extension<Identity>,
    WithRejection(Json(req), _): IRejection<Json<ReqCreate>>,
) -> Result<ApiOK<()>> {
    if let Err(err) = req.validate() {
        return Err(ApiErr::ErrParams(Some(err.to_string())));
    }

    crate::service::project::create(identity, req).await
}

pub async fn list(
    Extension(identity): Extension<Identity>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<ApiOK<RespList>> {
    crate::service::project::list(identity, query).await
}

pub async fn detail(
    Extension(identity): Extension<Identity>,
    Path(project_id): Path<u64>,
) -> Result<ApiOK<RespDetail>> {
    crate::service::project::detail(identity, project_id).await
}
