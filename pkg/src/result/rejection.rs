use axum::{
    extract::rejection::JsonRejection,
    response::{IntoResponse, Response},
};
use axum_extra::extract::WithRejection;
use thiserror::Error;

use super::response::ApiErr;

#[derive(Debug, Error)]
pub enum MyRejection {
    // The `#[from]` attribute generates `From<JsonRejection> for MyRejection`
    // implementation. See `thiserror` docs for more information
    #[error(transparent)]
    JSONExtractor(#[from] JsonRejection),
}

// We implement `IntoResponse` so MyRejection can be used as a response
impl IntoResponse for MyRejection {
    fn into_response(self) -> Response {
        let err = match self {
            MyRejection::JSONExtractor(x) => match x {
                JsonRejection::JsonDataError(e) => ApiErr::ErrData(Some(e.body_text())),
                JsonRejection::JsonSyntaxError(e) => ApiErr::ErrData(Some(e.body_text())),
                JsonRejection::MissingJsonContentType(e) => ApiErr::ErrData(Some(e.body_text())),
                _ => ApiErr::ErrSystem(None),
            },
        };
        err.into_response()
    }
}

pub type IRejection<T> = WithRejection<T, MyRejection>;
