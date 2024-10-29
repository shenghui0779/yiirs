use sea_orm::sea_query::Expr;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::app::model::{account, prelude::Account};
use crate::shared::core::db;
use crate::shared::crypto::hash;
use crate::shared::result::code::Code;
use crate::shared::result::{reply, ApiResult};
use crate::shared::util::identity::Identity;
use crate::shared::util::{helper, xtime};

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

pub async fn login(req: ReqLogin) -> ApiResult<RespLogin> {
    let model = Account::find()
        .filter(account::Column::Username.eq(req.username))
        .one(db::conn())
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "error find account");
            Code::ErrSystem(None)
        })?
        .ok_or(Code::ErrAuth(Some("账号不存在".to_string())))?;

    let pass = format!("{}{}", req.password, model.salt);
    if hash::md5(pass.as_bytes()) != model.password {
        return Err(Code::ErrAuth(Some("密码错误".to_string())));
    }

    let now = xtime::now(None).unix_timestamp();
    let login_token =
        hash::md5(format!("auth.{}.{}.{}", model.id, now, helper::nonce(16)).as_bytes());
    let auth_token = Identity::new(model.id, model.role, login_token.clone())
        .to_auth_token()
        .map_err(|e| {
            tracing::error!(error = ?e, "error identity encrypt");
            Code::ErrSystem(None)
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
        return Err(Code::ErrSystem(None));
    }

    let resp = RespLogin {
        name: model.realname,
        role: model.role,
        auth_token,
    };

    Ok(reply::OK(Some(resp)))
}

pub async fn logout(identity: &Identity) -> ApiResult<()> {
    let ret = Account::update_many()
        .filter(account::Column::Id.eq(identity.id()))
        .col_expr(account::Column::LoginToken, Expr::value(""))
        .col_expr(
            account::Column::CreatedAt,
            Expr::value(xtime::now(None).unix_timestamp()),
        )
        .exec(db::conn())
        .await;

    if let Err(e) = ret {
        tracing::error!(error = ?e, "error update account");
        return Err(Code::ErrSystem(None));
    }

    Ok(reply::OK(None))
}
