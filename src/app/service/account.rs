use std::collections::HashMap;

use multimap::MultiMap;
use sea_orm::{
    ColumnTrait, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::shared::core::db;
use crate::shared::crypto::hash;
use crate::shared::result::{status, ApiResult};
use crate::shared::util::{helper, xtime};

use crate::app::model::{account, prelude::Account};

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct ReqCreate {
    #[validate(length(min = 1, message = "用户名必填"))]
    pub username: String,
    #[validate(length(min = 1, message = "密码必填"))]
    pub password: String,
    #[validate(length(min = 1, message = "实名必填"))]
    pub realname: String,
}

pub async fn create(req: ReqCreate) -> ApiResult<()> {
    let count = Account::find()
        .filter(account::Column::Username.eq(req.username.clone()))
        .count(db::conn())
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "error find account");
            status::Err::System(None)
        })?;
    if count > 0 {
        return Err(status::Err::Perm(Some("该用户名已被使用".to_string())));
    }

    let salt = helper::nonce(16);
    let pass = format!("{}{}", req.password, salt);
    let now = xtime::now(None).unix_timestamp();
    let model = account::ActiveModel {
        username: Set(req.username),
        password: Set(hash::md5(pass.as_bytes())),
        salt: Set(salt),
        role: Set(1),
        realname: Set(req.realname),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    if let Err(e) = Account::insert(model).exec(db::conn()).await {
        tracing::error!(error = ?e, "error insert account");
        return Err(status::Err::System(None));
    }

    Ok(status::OK(None))
}

#[derive(Debug, Serialize)]
pub struct RespInfo {
    pub id: u64,
    pub username: String,
    pub realname: String,
    pub login_at: i64,
    pub login_at_str: String,
    pub created_at: i64,
    pub created_at_str: String,
}

pub async fn info(account_id: u64) -> ApiResult<RespInfo> {
    let model = Account::find_by_id(account_id)
        .one(db::conn())
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Error Account::find_by_id");
            status::Err::System(None)
        })?
        .ok_or(status::Err::NotFound(Some("账号不存在".to_string())))?;

    let resp = RespInfo {
        id: model.id,
        username: model.username,
        realname: model.realname,
        login_at: model.login_at,
        login_at_str: xtime::to_string(xtime::DATE_TIME, model.login_at, None).unwrap_or_default(),
        created_at: model.created_at,
        created_at_str: xtime::to_string(xtime::DATE_TIME, model.created_at, None)
            .unwrap_or_default(),
    };

    Ok(status::OK(Some(resp)))
}

#[derive(Debug, Serialize)]
pub struct RespList {
    pub total: i64,
    pub list: Vec<RespInfo>,
}

pub async fn list(query: &MultiMap<String, String>) -> ApiResult<RespList> {
    let mut builder = Account::find();
    if let Some(username) = query.get("username") {
        if !username.is_empty() {
            builder = builder.filter(account::Column::Username.eq(username.to_owned()));
        }
    }

    let mut total: i64 = 0;
    let (offset, limit) = helper::query_page(query);
    // 仅在第一页计算数量
    if offset == 0 {
        total = builder
            .clone()
            .select_only()
            .column_as(account::Column::Id.count(), "count")
            .into_tuple::<i64>()
            .one(db::conn())
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "error count account");
                status::Err::System(None)
            })?
            .unwrap_or_default();
    }

    let models = builder
        .order_by(account::Column::Id, Order::Desc)
        .offset(offset)
        .limit(limit)
        .all(db::conn())
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "error find account");
            status::Err::System(None)
        })?;
    let mut resp = RespList {
        total,
        list: (Vec::with_capacity(models.len())),
    };
    for model in models {
        let info = RespInfo {
            id: model.id,
            username: model.username,
            realname: model.realname,
            login_at: model.login_at,
            login_at_str: xtime::to_string(xtime::DATE_TIME, model.login_at, None)
                .unwrap_or_default(),
            created_at: model.created_at,
            created_at_str: xtime::to_string(xtime::DATE_TIME, model.created_at, None)
                .unwrap_or_default(),
        };
        resp.list.push(info);
    }

    Ok(status::OK(Some(resp)))
}
