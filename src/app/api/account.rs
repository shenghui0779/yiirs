use salvo::{handler, Request};
use validator::Validate;

use crate::shared::{
    result::{
        status::{self},
        ApiResult,
    },
    util::identity::{Identity, Role},
};

use crate::app::service::{
    self,
    account::{ReqCreate, RespInfo, RespList},
};

#[handler]
pub async fn create(req: &mut Request) -> ApiResult<()> {
    let params = req.parse_json::<ReqCreate>().await.map_err(|e| {
        tracing::error!(error = ?e, "Error req.parse_json");
        status::Err::Params(Some("参数解析出错".to_string()))
    })?;
    if let Err(e) = params.validate() {
        return Err(status::Err::Params(Some(e.to_string())));
    }

    let empty = Identity::empty();
    let id = req.extensions().get::<Identity>().unwrap_or(&empty);
    if !id.is_role(Role::Super) {
        return Err(status::Err::Perm(None));
    }

    service::account::create(params).await
}

#[handler]
pub async fn info(req: &mut Request) -> ApiResult<RespInfo> {
    let empty = Identity::empty();
    let id = req.extensions().get::<Identity>().unwrap_or(&empty);
    if !id.is_role(Role::Super) {
        return Err(status::Err::Perm(None));
    }
    let account_id = req.param::<u64>("account_id").unwrap_or_default();
    service::account::info(account_id).await
}

#[handler]
pub async fn list(req: &mut Request) -> ApiResult<RespList> {
    let empty = Identity::empty();
    let id = req.extensions().get::<Identity>().unwrap_or(&empty);
    if !id.is_role(Role::Super) {
        return Err(status::Err::Perm(None));
    }
    service::account::list(req.queries()).await
}
