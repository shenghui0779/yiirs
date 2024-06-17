use anyhow::anyhow;
use anyhow::Result;
use pkg::config;
use pkg::db;
use pkg::identity::Identity;
use sea_orm::EntityTrait;

use crate::ent::prelude::Account;

pub mod controller;
pub mod middleware;
pub mod router;
pub mod service;

pub async fn auth_check(identity: &Identity) -> Result<()> {
    if identity.id() == 0 {
        return Err(anyhow!("未授权，请先登录"));
    }
    let ret = Account::find_by_id(identity.id()).one(db::conn()).await?;
    match ret {
        None => return Err(anyhow!("授权账号不存在")),
        Some(v) => {
            if v.login_token.is_empty() || !identity.match_token(v.login_token) {
                return Err(anyhow!("授权已失效"));
            }
        }
    }
    Ok(())
}

pub async fn serve() {
    // run it with hyper on localhost:8000
    let addr = config::global().get_int("app.port").unwrap_or(8000);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", addr))
        .await
        .unwrap();

    tracing::info!("listening on {}", addr);

    axum::serve(listener, router::app::init()).await.unwrap();
}
