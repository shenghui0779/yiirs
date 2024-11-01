use salvo::{handler, Request};
use validator::Validate;

use crate::shared::{
    result::{code::Code, ApiResult},
    util::identity::Identity,
};

use crate::app::service::{
    self,
    project::{ReqCreate, RespDetail, RespList},
};

#[handler]
pub async fn create(req: &mut Request) -> ApiResult<()> {
    let params = req.parse_json::<ReqCreate>().await.map_err(|e| {
        tracing::error!(error = ?e, "Error req.parse_json");
        Code::ErrParams(Some("参数解析出错".to_string()))
    })?;
    if let Err(e) = params.validate() {
        return Err(Code::ErrParams(Some(e.to_string())));
    }

    let empty = Identity::empty();
    let id = req.extensions().get::<Identity>().unwrap_or(&empty);
    service::project::create(id, params).await
}

#[handler]
pub async fn list(req: &mut Request) -> ApiResult<RespList> {
    let empty = Identity::empty();
    let id = req.extensions().get::<Identity>().unwrap_or(&empty);
    service::project::list(id, req.queries()).await
}

#[handler]
pub async fn detail(req: &mut Request) -> ApiResult<RespDetail> {
    let empty = Identity::empty();
    let id = req.extensions().get::<Identity>().unwrap_or(&empty);
    let project_id = req.param::<u64>("project_id").unwrap_or_default();
    service::project::detail(id, project_id).await
}
