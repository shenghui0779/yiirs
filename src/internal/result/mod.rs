use code::Code;

pub mod code;
pub mod rejection;
pub mod reply;

pub type ApiResult<T> = Result<reply::OK<T>, Code>;
