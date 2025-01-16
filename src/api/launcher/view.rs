use axum::Extension;
use axum::extract::State;
use axum::response::IntoResponse;
use crate::core::AppState;
use crate::core::state::ReqContext;
use crate::utility::LibResult;

pub async fn launch_token(
    State(app_state): State<AppState>,
    Extension(ctx): Extension<ReqContext>,
) -> LibResult<impl IntoResponse>{
    Ok("")
}