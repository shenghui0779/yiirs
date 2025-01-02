use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct Reply<T>
where
    T: Serialize,
{
    pub code: i32,
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

pub struct OK<T>(pub Option<T>)
where
    T: Serialize;

impl<T> OK<T>
where
    T: Serialize,
{
    pub fn to_reply(self) -> Reply<T> {
        Reply {
            code: 0,
            msg: String::from("OK"),
            data: self.0,
        }
    }
}

impl<T> IntoResponse for OK<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        Json(self.to_reply()).into_response()
    }
}
