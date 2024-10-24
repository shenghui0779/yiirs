pub mod code;
pub mod status;

pub type ApiResult<T> = Result<status::OK<T>, code::Code>;
