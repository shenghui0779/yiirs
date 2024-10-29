pub mod code;
pub mod reply;

pub type ApiResult<T> = Result<reply::OK<T>, code::Code>;
