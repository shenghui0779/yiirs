use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

use super::status::Status;

pub struct ApiOK<T>(pub Option<T>)
where
    T: Serialize;

impl<T> IntoResponse for ApiOK<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let ApiOK(data) = self;
        let status = Status::OK(data);

        Json(status.to_reply()).into_response()
    }
}

#[allow(dead_code)]
pub enum ApiErr {
    Error(i32, String),
    ErrParams(Option<String>),
    ErrAuth(Option<String>),
    ErrPerm(Option<String>),
    ErrNotFound(Option<String>),
    ErrSystem(Option<String>),
    ErrData(Option<String>),
    ErrService(Option<String>),
}

use ApiErr::*;

impl IntoResponse for ApiErr {
    fn into_response(self) -> Response {
        let status = match self {
            Error(code, msg) => Status::<()>::Err(code, msg),
            ErrParams(msg) => {
                let code = 10000;

                match msg {
                    Some(v) => Status::<()>::Err(code, v),
                    None => Status::<()>::Err(code, String::from("参数错误")),
                }
            }
            ErrAuth(msg) => {
                let code = 20000;

                match msg {
                    Some(v) => Status::<()>::Err(code, v),
                    None => Status::<()>::Err(code, String::from("未授权，请先登录")),
                }
            }
            ErrPerm(msg) => {
                let code = 30000;

                match msg {
                    Some(v) => Status::<()>::Err(code, v),
                    None => Status::<()>::Err(code, String::from("权限不足")),
                }
            }
            ErrNotFound(msg) => {
                let code = 40000;

                match msg {
                    Some(v) => Status::<()>::Err(code, v),
                    None => Status::<()>::Err(code, String::from("数据不存在")),
                }
            }
            ErrSystem(msg) => {
                let code = 50000;

                match msg {
                    Some(v) => Status::<()>::Err(code, v),
                    None => Status::<()>::Err(code, String::from("内部服务器错误，请稍后重试")),
                }
            }
            ErrData(msg) => {
                let code = 60000;

                match msg {
                    Some(v) => Status::<()>::Err(code, v),
                    None => Status::<()>::Err(code, String::from("数据异常")),
                }
            }
            ErrService(msg) => {
                let code = 70000;

                match msg {
                    Some(v) => Status::<()>::Err(code, v),
                    None => Status::<()>::Err(code, String::from("服务异常")),
                }
            }
        };

        Json(status.to_reply()).into_response()
    }
}

pub type Result<T> = std::result::Result<T, ApiErr>;
