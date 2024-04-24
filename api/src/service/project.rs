use std::collections::HashMap;

use entity::{prelude::*, project};
use sea_orm::{
    ColumnTrait, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use library::{core::db, time::Layout};
use library::{time, util};

use super::identity::{Identity, Role};
use crate::result::response::{ApiErr, ApiOK, Result};

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct ReqCreate {
    #[validate(length(min = 1, max = 8, message = "项目编号必填且长度最大为8"))]
    pub code: String,
    #[validate(length(min = 1, message = "项目名称必填"))]
    pub name: String,
    pub remark: Option<String>,
}

pub async fn create(identity: Identity, req: ReqCreate) -> Result<ApiOK<()>> {
    // 校验编号唯一性
    match Project::find()
        .filter(project::Column::Code.eq(req.code.clone()))
        .count(db::conn())
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
        code: Set(req.code),
        name: Set(req.name),
        remark: Set(req.remark.unwrap_or_default()),
        account_id: Set(identity.id()),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    if let Err(err) = Project::insert(am).exec(db::conn()).await {
        tracing::error!(error = ?err, "err insert project");
        return Err(ApiErr::ErrSystem(None));
    }

    Ok(ApiOK(None))
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

pub async fn list(identity: Identity, query: HashMap<String, String>) -> Result<ApiOK<RespList>> {
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

    let mut total: i64 = 0;

    // 仅在第一页计算数量
    if offset == 0 {
        total = match builder
            .clone()
            .select_only()
            .column_as(project::Column::Id.count(), "count")
            .into_tuple::<i64>()
            .one(db::conn())
            .await
        {
            Err(err) => {
                tracing::error!(error = ?err, "err count project");
                return Err(ApiErr::ErrSystem(None));
            }
            Ok(v) => v.unwrap_or_default(),
        }
    }

    let models = match builder
        .order_by(project::Column::Id, Order::Desc)
        .offset(offset)
        .limit(limit)
        .all(db::conn())
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
        let info = RespInfo {
            id: model.id,
            code: model.code,
            name: model.name,
            created_at: model.created_at,
            created_at_str: time::Format(Layout::DateTime(None)).to_string(model.created_at),
        };

        resp.list.push(info);
    }

    Ok(ApiOK(Some(resp)))
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

pub async fn detail(identity: Identity, project_id: u64) -> Result<ApiOK<RespDetail>> {
    let (model_proj, model_account) = match Project::find_by_id(project_id)
        .find_also_related(Account)
        .one(db::conn())
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
        created_at_str: time::Format(Layout::DateTime(None)).to_string(model_proj.created_at),
        account: None,
    };

    if let Some(v) = model_account {
        resp.account = Some(ProjAccount {
            id: v.id,
            name: v.realname,
        })
    }

    Ok(ApiOK(Some(resp)))
}
