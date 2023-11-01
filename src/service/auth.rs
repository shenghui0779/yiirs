use axum::extract::State;
use axum::{Extension, Json};
use axum_extra::extract::WithRejection;
use sea_orm::sea_query::Expr;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::config;
use crate::util::helper;
use crate::{
    entity::{account, prelude::*},
    result::{
        rejection::IRejection,
        response::{ApiErr, ApiOK, Result},
    },
    util::{auth::Identity, hash::md5},
};

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct ParamsLogin {
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

pub async fn login(
    State(state): State<config::AppState>,
    WithRejection(Json(params), _): IRejection<Json<ParamsLogin>>,
) -> Result<ApiOK<RespLogin>> {
    if let Err(err) = params.validate() {
        return Err(ApiErr::ErrParams(Some(err.to_string())));
    }

    let ret = Account::find()
        .filter(account::Column::Username.eq(params.username))
        .one(&state.db)
        .await;

    let record = match ret {
        Err(err) => {
            tracing::error!(error = ?err, "err find account");
            return Err(ApiErr::ErrSystem(None));
        }
        Ok(v) => v,
    };

    let model = match record {
        None => return Err(ApiErr::ErrAuth(Some("账号不存在".to_string()))),
        Some(v) => v,
    };

    let pass = format!("{}{}", params.password, model.salt);

    if md5(pass.as_bytes()) != model.password {
        return Err(ApiErr::ErrAuth(Some("密码错误".to_string())));
    }

    let now = chrono::Local::now().timestamp();
    let login_token = md5(format!("auth.{}.{}.{}", model.id, now, helper::nonce(16)).as_bytes());

    let auth_token = match Identity::new(model.id, model.role, login_token.clone()).to_auth_token()
    {
        Err(err) => {
            tracing::error!(error = ?err, "err identity encrypt");
            return Err(ApiErr::ErrSystem(None));
        }
        Ok(v) => v,
    };

    let am = account::ActiveModel {
        login_at: Set(now),
        login_token: Set(login_token),
        updated_at: Set(now),
        ..Default::default()
    };

    let ret_update = Account::update_many()
        .filter(account::Column::Id.eq(model.id))
        .set(am)
        .exec(&state.db)
        .await;

    if let Err(err) = ret_update {
        tracing::error!(error = ?err, "err update account");
        return Err(ApiErr::ErrSystem(None));
    }

    let resp = RespLogin {
        name: model.realname,
        role: model.role,
        auth_token,
    };

    Ok(ApiOK(Some(resp)))
}

pub async fn logout(
    State(state): State<config::AppState>,
    Extension(identity): Extension<Identity>,
) -> Result<ApiOK<()>> {
    if identity.id() == 0 {
        return Ok(ApiOK(None));
    }

    let ret = Account::update_many()
        .filter(account::Column::Id.eq(identity.id()))
        .col_expr(account::Column::LoginToken, Expr::value(""))
        .col_expr(
            account::Column::CreatedAt,
            Expr::value(chrono::Local::now().timestamp()),
        )
        .exec(&state.db)
        .await;

    if let Err(err) = ret {
        tracing::error!(error = ?err, "err update account");
        return Err(ApiErr::ErrSystem(None));
    }

    Ok(ApiOK(None))
}
