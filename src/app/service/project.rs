use std::collections::HashMap;

use sea_orm::{
    ColumnTrait, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::internal::core::db;
use crate::internal::result::code::Code;
use crate::internal::result::{reply, ApiResult};
use crate::internal::util::identity::{Identity, Role};
use crate::internal::util::{helper, xtime};

use crate::app::model::prelude::{Account, Project};
use crate::app::model::project;

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct ReqCreate {
    #[validate(length(min = 1, max = 8, message = "项目编号必填且长度最大为8"))]
    pub code: String,
    #[validate(length(min = 1, message = "项目名称必填"))]
    pub name: String,
    pub remark: Option<String>,
}

pub async fn create(id: Identity, req: ReqCreate) -> ApiResult<()> {
    // 校验编号唯一性
    let count = Project::find()
        .filter(project::Column::Code.eq(req.code.clone()))
        .count(db::conn())
        .await
        .map_err(|e| {
            tracing::error!(err = ?e, "find project");
            Code::ErrSystem(None)
        })?;
    if count > 0 {
        return Err(Code::ErrPerm(Some("该编号已被使用".to_string())));
    }

    let now = xtime::now(None).unix_timestamp();
    let model = project::ActiveModel {
        code: Set(req.code),
        name: Set(req.name),
        remark: Set(req.remark.unwrap_or_default()),
        account_id: Set(id.id()),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    if let Err(e) = Project::insert(model).exec(db::conn()).await {
        tracing::error!(err = ?e, "insert project");
        return Err(Code::ErrSystem(None));
    }

    Ok(reply::OK(None))
}

#[derive(Debug, Serialize)]
pub struct RespInfo {
    pub id: u64,
    pub code: String,
    pub name: String,
    pub created_at: i64,
    pub created_at_str: String,
}

#[derive(Debug, Serialize)]
pub struct RespList {
    pub total: i64,
    pub list: Vec<RespInfo>,
}

pub async fn list(id: Identity, query: HashMap<String, String>) -> ApiResult<RespList> {
    let mut builder = Project::find();
    if id.is_role(Role::Super) {
        if let Some(account_id) = query.get("account_id") {
            if let Ok(v) = account_id.parse::<u64>() {
                builder = builder.filter(project::Column::AccountId.eq(v));
            }
        }
    } else {
        builder = builder.filter(project::Column::AccountId.eq(id.id()));
    }
    if let Some(code) = query.get("code") {
        if !code.is_empty() {
            builder = builder.filter(project::Column::Code.eq(code.to_owned()));
        }
    }
    if let Some(name) = query.get("name") {
        if !name.is_empty() {
            builder = builder.filter(project::Column::Name.contains(name));
        }
    }

    let mut total: i64 = 0;
    let (offset, limit) = helper::query_page(&query);
    // 仅在第一页计算数量
    if offset == 0 {
        total = builder
            .clone()
            .select_only()
            .column_as(project::Column::Id.count(), "count")
            .into_tuple::<i64>()
            .one(db::conn())
            .await
            .map_err(|e| {
                tracing::error!(err = ?e, "count project");
                Code::ErrSystem(None)
            })?
            .unwrap_or_default();
    }

    let models = builder
        .order_by(project::Column::Id, Order::Desc)
        .offset(offset)
        .limit(limit)
        .all(db::conn())
        .await
        .map_err(|e| {
            tracing::error!(err = ?e, "find project");
            Code::ErrSystem(None)
        })?;
    let mut resp = RespList {
        total,
        list: (Vec::with_capacity(models.len())),
    };
    for model in models {
        let info = RespInfo {
            id: model.id,
            code: model.code,
            name: model.name,
            created_at: model.created_at,
            created_at_str: xtime::to_string(xtime::DATE_TIME, model.created_at, None)
                .unwrap_or_default(),
        };
        resp.list.push(info);
    }

    Ok(reply::OK(Some(resp)))
}

#[derive(Debug, Serialize)]
pub struct RespDetail {
    pub id: u64,
    pub code: String,
    pub name: String,
    pub created_at: i64,
    pub created_at_str: String,
    pub account: Option<ProjAccount>,
}

#[derive(Debug, Serialize)]
pub struct ProjAccount {
    pub id: u64,
    pub name: String,
}

pub async fn detail(id: Identity, project_id: u64) -> ApiResult<RespDetail> {
    let (model_proj, model_account) = Project::find_by_id(project_id)
        .find_also_related(Account)
        .one(db::conn())
        .await
        .map_err(|e| {
            tracing::error!(err = ?e, "find project");
            Code::ErrSystem(None)
        })?
        .ok_or(Code::ErrEmpty(Some("项目不存在".to_string())))?;
    if !id.is_role(Role::Super) && id.id() != model_proj.account_id {
        return Err(Code::ErrPerm(None));
    }

    let mut resp = RespDetail {
        id: model_proj.id,
        code: model_proj.code,
        name: model_proj.name,
        created_at: model_proj.created_at,
        created_at_str: xtime::to_string(xtime::DATE_TIME, model_proj.created_at, None)
            .unwrap_or_default(),
        account: None,
    };
    if let Some(v) = model_account {
        resp.account = Some(ProjAccount {
            id: v.id,
            name: v.username,
        })
    }

    Ok(reply::OK(Some(resp)))
}
