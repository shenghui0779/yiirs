use response::ApiErr;

pub mod rejection;
pub mod response;
pub mod status;

pub type Result<T> = std::result::Result<T, ApiErr>;
