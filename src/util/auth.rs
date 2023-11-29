use anyhow::{anyhow, Result};
use base64::{prelude::BASE64_STANDARD, Engine};
use config::Config;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};

use crate::{crypto::aes::CBC, entity::prelude::Account};

#[allow(dead_code)]
pub enum Role {
    Super,
    Normal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    pub fn empty() -> Self {
        Self {
            i: 0,
            r: 0,
            t: String::from(""),
        }
    }

    pub fn from_auth_token(cfg: &Config, token: String) -> Self {
        let cipher = match BASE64_STANDARD.decode(token) {
            Err(err) => {
                tracing::error!(error = ?err, "err invalid auth_token");
                return Identity::empty();
            }
            Ok(v) => v,
        };

        let secret = match cfg.get_string("app.secret") {
            Err(err) => {
                tracing::error!(error = ?err, "err missing config(app.secret)");
                return Identity::empty();
            }
            Ok(v) => v,
        };
        let key = secret.as_bytes();

        let plain = match CBC(key, &key[..16]).decrypt(&cipher) {
            Err(err) => {
                tracing::error!(error = ?err, "err invalid auth_token");
                return Identity::empty();
            }
            Ok(v) => v,
        };

        match serde_json::from_slice::<Identity>(&plain) {
            Err(err) => {
                tracing::error!(error = ?err, "err invalid auth_token");
                return Identity::empty();
            }
            Ok(identity) => identity,
        }
    }

    pub fn to_auth_token(&self, cfg: &Config) -> Result<String> {
        let secret = cfg.get_string("app.secret")?;
        let key = secret.as_bytes();

        let plain = serde_json::to_vec(self)?;
        let cipher = CBC(key, &key[..16]).encrypt(&plain, None)?;

        Ok(BASE64_STANDARD.encode(cipher))
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

    pub async fn check(&self, db: &DatabaseConnection) -> Result<()> {
        if self.id() == 0 {
            return Err(anyhow!("未授权，请先登录"));
        }

        match Account::find_by_id(self.id()).one(db).await? {
            None => return Err(anyhow!("授权账号不存在")),
            Some(v) => {
                if v.login_token.len() == 0 || self.t != v.login_token {
                    return Err(anyhow!("授权已失效"));
                }
            }
        }

        Ok(())
    }

    pub fn to_string(&self) -> String {
        if self.i == 0 {
            return String::from("<none>");
        }

        if self.r == 0 {
            return format!("id:{}|token:{}", self.i, self.t);
        }

        format!("id:{}|role:{}|token:{}", self.i, self.r, self.t)
    }
}
