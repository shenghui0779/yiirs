use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};

use library::result::response::ApiErr;

use crate::{auth::identity::Identity, AppState};

pub async fn handle(State(state): State<AppState>, request: Request, next: Next) -> Response {
    let identity = request.extensions().get::<Identity>();

    match identity {
        None => return ApiErr::ErrAuth(None).into_response(),
        Some(v) => match v.check(&state.db).await {
            Ok(_) => (),
            Err(err) => return ApiErr::ErrAuth(Some(err.to_string())).into_response(),
        },
    }

    next.run(request).await
}
