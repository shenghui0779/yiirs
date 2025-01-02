use axum::{
    response::{IntoResponse, Response},
    Json,
};

use super::reply::Reply;

pub enum Code {
    New(i32, String),
    ErrParams(Option<String>),
    ErrAuth(Option<String>),
    ErrPerm(Option<String>),
    ErrEmpty(Option<String>),
    ErrSystem(Option<String>),
    ErrData(Option<String>),
    ErrService(Option<String>),
}

impl Code {
    pub fn to_reply(self) -> Reply<()> {
        let (code, msg) = match self {
            Code::New(code, msg) => (code, msg),
            Code::ErrParams(msg) => (10000, msg.unwrap_or(String::from("参数错误"))),
            Code::ErrAuth(msg) => (20000, msg.unwrap_or(String::from("未授权，请先登录"))),
            Code::ErrPerm(msg) => (30000, msg.unwrap_or(String::from("权限不足"))),
            Code::ErrEmpty(msg) => (40000, msg.unwrap_or(String::from("数据不存在"))),
            Code::ErrSystem(msg) => (50000, msg.unwrap_or(String::from("内部服务器错误"))),
            Code::ErrData(msg) => (60000, msg.unwrap_or(String::from("数据异常"))),
            Code::ErrService(msg) => (70000, msg.unwrap_or(String::from("服务异常"))),
        };
        Reply {
            code,
            msg,
            data: None,
        }
    }
}

impl IntoResponse for Code {
    fn into_response(self) -> Response {
        Json(self.to_reply()).into_response()
    }
}
