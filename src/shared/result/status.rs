use salvo::prelude::*;
use salvo::{Depot, Request, Response, Writer};
use serde::Serialize;

#[derive(Serialize)]
pub struct Reply<T>
where
    T: Serialize,
{
    pub code: i32,
    pub err: bool,
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

pub enum Status<T>
where
    T: Serialize,
{
    OK(Option<T>),
    Err(i32, String),
}

impl<T> Status<T>
where
    T: Serialize,
{
    pub fn to_reply(self) -> Reply<T> {
        let mut resp = Reply {
            code: 0,
            err: false,
            msg: String::from("OK"),
            data: None,
        };
        match self {
            Status::OK(data) => {
                resp.data = data;
            }
            Status::Err(code, msg) => {
                resp.code = code;
                resp.err = true;
                resp.msg = msg;
            }
        }
        resp
    }
}

pub struct OK<T>(pub Option<T>)
where
    T: Serialize;

impl<T> OK<T>
where
    T: Serialize + std::marker::Send,
{
    fn to_status(self) -> Status<T> {
        let OK(data) = self;
        Status::OK(data)
    }
}

#[async_trait]
impl<T> Writer for OK<T>
where
    T: Serialize + std::marker::Send,
{
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.render(Json(self.to_status().to_reply()));
    }
}

pub enum Err {
    New(i32, String),
    Params(Option<String>),
    Auth(Option<String>),
    Perm(Option<String>),
    NotFound(Option<String>),
    System(Option<String>),
    Data(Option<String>),
    Service(Option<String>),
}

impl Err {
    pub fn to_status(self) -> Status<()> {
        let status: Status<()> = match self {
            Err::New(code, msg) => Status::Err(code, msg),
            Err::Params(msg) => Status::Err(10000, msg.unwrap_or(String::from("参数错误"))),
            Err::Auth(msg) => Status::Err(20000, msg.unwrap_or(String::from("未授权，请先登录"))),
            Err::Perm(msg) => Status::Err(30000, msg.unwrap_or(String::from("权限不足"))),
            Err::NotFound(msg) => Status::Err(40000, msg.unwrap_or(String::from("数据不存在"))),
            Err::System(msg) => Status::Err(
                50000,
                msg.unwrap_or(String::from("内部服务器错误，请稍后重试")),
            ),
            Err::Data(msg) => Status::Err(60000, msg.unwrap_or(String::from("数据异常"))),
            Err::Service(msg) => Status::Err(70000, msg.unwrap_or(String::from("服务异常"))),
        };
        status
    }
}

#[async_trait]
impl Writer for Err {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.render(Json(self.to_status().to_reply()));
    }
}
