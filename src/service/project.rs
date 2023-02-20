use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    http::HeaderMap,
    Json,
};
use axum_extra::extract::WithRejection;
use chrono::prelude::*;
use sea_orm::{
    ColumnTrait, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    config::db,
    entity::{prelude::*, project},
    result::{
        rejection::IRejection,
        response::{ApiData, ApiErr, Result},
    },
    util::{
        self,
        auth::{self, Role},
    },
};

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct ParamsCreate {
    #[validate(length(min = 1, max = 8, message = "项目编号必填且长度最大为8"))]
    pub code: String,
    #[validate(length(min = 1, message = "项目名称必填"))]
    pub name: String,
    pub remark: Option<String>,
}

pub async fn create(
    headers: HeaderMap,
    WithRejection(Json(params), _): IRejection<Json<ParamsCreate>>,
) -> Result<ApiData<()>> {
    if let Err(err) = params.validate() {
        return Err(ApiErr::ErrParams(Some(err.to_string())));
    }

    let identity = match auth::check(headers, Some(Role::Normal)).await {
        Err(err) => return Err(ApiErr::ErrAuth(Some(err.to_string()))),
        Ok(v) => v,
    };

    // 校验编号唯一性
    match Project::find()
        .filter(project::Column::Code.eq(params.code.clone()))
        .count(db::get())
        .await
    {
        Err(err) => {
            tracing::error!(error = ?err, "err find project");
            return Err(ApiErr::ErrSystem(None));
        }
        Ok(v) => {
            if v > 0 {
                return Err(ApiErr::ErrPerm(Some("该编号已被使用".to_string())));
            }
        }
    }

    let now = chrono::Local::now().timestamp();

    let am = project::ActiveModel {
        code: Set(params.code),
        name: Set(params.name),
        remark: Set(params.remark.unwrap_or_default()),
        account_id: Set(identity.id()),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    if let Err(err) = Project::insert(am).exec(db::get()).await {
        tracing::error!(error = ?err, "err insert project");
        return Err(ApiErr::ErrSystem(None));
    }

    Ok(ApiData(None))
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
    pub total: u64,
    pub list: Vec<RespInfo>,
}

pub async fn list(
    headers: HeaderMap,
    Query(query): Query<HashMap<String, String>>,
) -> Result<ApiData<RespList>> {
    let identity = match auth::check(headers, Some(Role::Normal)).await {
        Err(err) => return Err(ApiErr::ErrAuth(Some(err.to_string()))),
        Ok(v) => v,
    };

    let mut builder = Project::find();

    if identity.is_role(Role::Super) {
        if let Some(account_id) = query.get("account_id") {
            if let Ok(v) = account_id.parse::<u64>() {
                builder = builder.filter(project::Column::AccountId.eq(v));
            }
        }
    } else {
        builder = builder.filter(project::Column::AccountId.eq(identity.id()));
    }

    if let Some(code) = query.get("code") {
        if code.len() > 0 {
            builder = builder.filter(project::Column::Code.eq(code.to_owned()));
        }
    }

    if let Some(name) = query.get("name") {
        if name.len() > 0 {
            builder = builder.filter(project::Column::Name.contains(name));
        }
    }

    let (offset, limit) = util::query_page(&query);

    let mut total: u64 = 0;

    // 仅在第一页计算数量
    if offset == 0 {
        let ret = builder.clone().count(db::get()).await;

        total = match ret {
            Err(err) => {
                tracing::error!(error = ?err, "err count project");
                return Err(ApiErr::ErrSystem(None));
            }
            Ok(v) => v,
        }
    }

    let models = match builder
        .order_by(project::Column::Id, Order::Desc)
        .offset(offset)
        .limit(limit)
        .all(db::get())
        .await
    {
        Err(err) => {
            tracing::error!(error = ?err, "err find project");
            return Err(ApiErr::ErrSystem(None));
        }
        Ok(v) => v,
    };

    let mut resp = RespList {
        total,
        list: (Vec::with_capacity(models.len())),
    };

    for model in models {
        let mut info = RespInfo {
            id: model.id,
            code: model.code,
            name: model.name,
            created_at: model.created_at,
            created_at_str: String::from(""),
        };

        if let Some(v) = Local.timestamp_opt(model.created_at, 0).single() {
            info.created_at_str = v.format("%Y-%m-%d %H:%M:%S").to_string()
        }

        resp.list.push(info);
    }

    Ok(ApiData(Some(resp)))
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

pub async fn detail(
    headers: HeaderMap,
    Path(project_id): Path<u64>,
) -> Result<ApiData<RespDetail>> {
    let identity = match auth::check(headers, Some(Role::Normal)).await {
        Err(err) => return Err(ApiErr::ErrAuth(Some(err.to_string()))),
        Ok(v) => v,
    };

    let (model_proj, model_account) = match Project::find_by_id(project_id)
        .find_also_related(Account)
        .one(db::get())
        .await
    {
        Err(err) => {
            tracing::error!(error = ?err, "err find project");
            return Err(ApiErr::ErrSystem(None));
        }
        Ok(v) => match v {
            None => return Err(ApiErr::ErrNotFound(Some("项目不存在".to_string()))),
            Some((model_proj, model_account)) => {
                if !identity.is_role(Role::Super) && identity.id() != model_proj.account_id {
                    return Err(ApiErr::ErrPerm(None));
                }

                (model_proj, model_account)
            }
        },
    };

    let mut resp = RespDetail {
        id: model_proj.id,
        code: model_proj.code,
        name: model_proj.name,
        created_at: model_proj.created_at,
        created_at_str: String::from(""),
        account: None,
    };

    if let Some(v) = Local.timestamp_opt(model_proj.created_at, 0).single() {
        resp.created_at_str = v.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    if let Some(v) = model_account {
        resp.account = Some(ProjAccount {
            id: v.id,
            name: v.realname,
        })
    }

    Ok(ApiData(Some(resp)))
}
