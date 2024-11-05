use crate::app::model::prelude::Account;
use crate::shared::core::db;
use crate::shared::util::identity::Identity;
use anyhow::anyhow;
use anyhow::Result;
use sea_orm::EntityTrait;

pub mod auth;

pub async fn auth_check(identity: &Identity) -> Result<()> {
    if identity.id() == 0 {
        return Err(anyhow!("未授权，请先登录"));
    }
    let ret = Account::find_by_id(identity.id()).one(db::conn()).await?;
    match ret {
        None => return Err(anyhow!("授权账号不存在")),
        Some(v) => {
            if v.login_token.is_empty()
                || !identity.match_token(v.login_token)
                || identity.is_expired()
            {
                return Err(anyhow!("授权已失效"));
            }
        }
    }
    Ok(())
}
