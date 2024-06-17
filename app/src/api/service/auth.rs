use sea_orm::sea_query::Expr;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use time::macros::offset;
use validator::Validate;

use pkg::crypto::hash::md5;
use pkg::identity::Identity;
use pkg::result::response::{ApiErr, ApiOK, Result};
use pkg::{db, util, xtime};

use crate::ent::account;
use crate::ent::prelude::Account;

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct ReqLogin {
    #[validate(length(min = 1, message = "用户名必填"))]
    pub username: String,
    #[validate(length(min = 1, message = "密码必填"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RespLogin {
    pub name: String,
    pub role: i8,
    pub auth_token: String,
}

pub async fn login(req: ReqLogin) -> Result<ApiOK<RespLogin>> {
    let model = Account::find()
        .filter(account::Column::Username.eq(req.username))
        .one(db::conn())
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "error find account");
            ApiErr::ErrSystem(None)
        })?
        .ok_or(ApiErr::ErrAuth(Some("账号不存在".to_string())))?;

    let pass = format!("{}{}", req.password, model.salt);
    if md5(pass.as_bytes()) != model.password {
        return Err(ApiErr::ErrAuth(Some("密码错误".to_string())));
    }

    let now = xtime::now(offset!(+8)).unix_timestamp();
    let login_token = md5(format!("auth.{}.{}.{}", model.id, now, util::nonce(16)).as_bytes());
    let auth_token = Identity::new(model.id, model.role, login_token.clone())
        .to_auth_token()
        .map_err(|e| {
            tracing::error!(error = ?e, "error identity encrypt");
            ApiErr::ErrSystem(None)
        })?;
    let update_model = account::ActiveModel {
        login_at: Set(now),
        login_token: Set(login_token),
        updated_at: Set(now),
        ..Default::default()
    };
    let ret_update = Account::update_many()
        .filter(account::Column::Id.eq(model.id))
        .set(update_model)
        .exec(db::conn())
        .await;
    if let Err(e) = ret_update {
        tracing::error!(error = ?e, "error update account");
        return Err(ApiErr::ErrSystem(None));
    }

    let resp = RespLogin {
        name: model.realname,
        role: model.role,
        auth_token,
    };

    Ok(ApiOK(Some(resp)))
}

pub async fn logout(identity: Identity) -> Result<ApiOK<()>> {
    let ret = Account::update_many()
        .filter(account::Column::Id.eq(identity.id()))
        .col_expr(account::Column::LoginToken, Expr::value(""))
        .col_expr(
            account::Column::CreatedAt,
            Expr::value(xtime::now(offset!(+8)).unix_timestamp()),
        )
        .exec(db::conn())
        .await;

    if let Err(e) = ret {
        tracing::error!(error = ?e, "error update account");
        return Err(ApiErr::ErrSystem(None));
    }

    Ok(ApiOK(None))
}
