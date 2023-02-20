use std::env;

use anyhow::{anyhow, Ok, Result};
use axum::http::HeaderMap;
use base64::{prelude::BASE64_STANDARD, Engine};
use crypto::aes::KeySize::KeySize256;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

use crate::{config::db, entity::prelude::Account};

use super::crypto::AES;

pub enum Role {
    Super,
    Normal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identity {
    i: u64,
    r: i8,
    t: String,
}

impl Identity {
    pub fn new(id: u64, role: i8, token: String) -> Self {
        Self {
            i: id,
            r: role,
            t: token,
        }
    }
    pub fn id(&self) -> u64 {
        self.i
    }
    pub fn is_role(&self, role: Role) -> bool {
        match role {
            Role::Normal => {
                if self.r == 1 {
                    return true;
                }
            }
            Role::Super => {
                if self.r == 2 {
                    return true;
                }
            }
        }

        false
    }
    pub fn match_role(&self, role: Role) -> bool {
        match role {
            Role::Normal => {
                if self.r >= 1 {
                    return true;
                }
            }
            Role::Super => {
                if self.r >= 2 {
                    return true;
                }
            }
        }

        false
    }
    pub fn match_token(&self, login_token: &str) -> bool {
        if self.t == login_token {
            return true;
        }

        false
    }
    pub fn decrypt(s: String) -> Result<Identity> {
        let cipher = BASE64_STANDARD.decode(s)?;

        let secret = env::var("API_SECRET")?;
        let key = secret.as_bytes();

        let plain = AES::CBC(KeySize256, key, &key[..16]).decrypt(&cipher)?;

        let identity: Identity = serde_json::from_slice(&plain)?;

        Ok(identity)
    }

    pub fn encrypt(&self) -> Result<String> {
        let secret = env::var("API_SECRET")?;
        let key = secret.as_bytes();

        let plain = serde_json::to_vec(self)?;

        let cipher = AES::CBC(KeySize256, key, &key[..16]).encrypt(&plain)?;

        Ok(BASE64_STANDARD.encode(cipher))
    }
}

pub async fn check(headers: HeaderMap, role: Option<Role>) -> Result<Identity> {
    let token = match headers.get("authorization") {
        None => return Err(anyhow!("未授权，请先登录")),
        Some(v) => v.to_str()?.to_string(),
    };

    let identity = Identity::decrypt(token)?;

    if identity.id() == 0 {
        return Err(anyhow!("未授权，请先登录"));
    }

    if let Some(v) = role {
        if !identity.match_role(v) {
            return Err(anyhow!("权限不足"));
        }
    }

    match Account::find_by_id(identity.id()).one(db::get()).await? {
        None => return Err(anyhow!("授权账号不存在")),
        Some(v) => {
            if v.login_token.len() == 0 || !identity.match_token(&v.login_token) {
                return Err(anyhow!("授权已失效"));
            }
        }
    }

    Ok(identity)
}
