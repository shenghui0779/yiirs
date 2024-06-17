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
        let status: Status<()> = match self {
            Error(code, msg) => Status::Err(code, msg),
            ErrParams(msg) => Status::Err(10000, msg.unwrap_or(String::from("参数错误"))),
            ErrAuth(msg) => Status::Err(20000, msg.unwrap_or(String::from("未授权，请先登录"))),
            ErrPerm(msg) => Status::Err(30000, msg.unwrap_or(String::from("权限不足"))),
            ErrNotFound(msg) => Status::Err(40000, msg.unwrap_or(String::from("数据不存在"))),
            ErrSystem(msg) => Status::Err(
                50000,
                msg.unwrap_or(String::from("内部服务器错误，请稍后重试")),
            ),
            ErrData(msg) => Status::Err(60000, msg.unwrap_or(String::from("数据异常"))),
            ErrService(msg) => Status::Err(70000, msg.unwrap_or(String::from("服务异常"))),
        };
        Json(status.to_reply()).into_response()
    }
}

pub type Result<T> = std::result::Result<T, ApiErr>;
