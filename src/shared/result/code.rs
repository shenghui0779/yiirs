use salvo::prelude::*;
use salvo::{Depot, Request, Response, Writer};

use super::status::Status;

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
    pub fn to_status(self) -> Status<()> {
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
        Status {
            code,
            err: true,
            msg,
            data: None,
        }
    }
}

#[async_trait]
impl Writer for Code {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, resp: &mut Response) {
        resp.render(Json(self.to_status()));
    }
}
