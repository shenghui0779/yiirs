pub mod status;

pub type ApiResult<T> = Result<status::OK<T>, status::Err>;
