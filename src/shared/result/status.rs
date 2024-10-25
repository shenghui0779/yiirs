use salvo::prelude::*;
use salvo::{Depot, Request, Response, Writer};
use serde::Serialize;

#[derive(Serialize)]
pub struct Status<T>
where
    T: Serialize,
{
    pub code: i32,
    pub err: bool,
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

pub struct OK<T>(pub Option<T>)
where
    T: Serialize;

impl<T> OK<T>
where
    T: Serialize + std::marker::Send,
{
    pub fn to_status(self) -> Status<T> {
        Status {
            code: 0,
            err: false,
            msg: String::from("OK"),
            data: self.0,
        }
    }
}

#[async_trait]
impl<T> Writer for OK<T>
where
    T: Serialize + std::marker::Send,
{
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, resp: &mut Response) {
        resp.render(Json(self.to_status()));
    }
}
