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
