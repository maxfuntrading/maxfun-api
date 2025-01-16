use axum::Extension;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use crate::api::profile::{logic, schema};
use crate::core::AppState;
use crate::core::state::ReqContext;
use crate::utility::{LibResult, Resp200};

pub async fn userinfo(
    State(app_state): State<AppState>,
    Extension(ctx): Extension<ReqContext>,
) -> LibResult<impl IntoResponse> {
    let rsp = logic::get_user_info(app_state, ctx.user_addr).await?;
    Ok(Resp200::new(rsp))
}

pub async fn get_token_owned(
    State(app_state): State<AppState>,
    Extension(ctx): Extension<ReqContext>,
    Query(params): Query<schema::TokenOwnedQuery>,
) -> LibResult<impl IntoResponse> {
let rsp = logic::get_token_owned(app_state, ctx.user_addr, params).await?;
    Ok(Resp200::new(rsp))
}

pub async fn get_token_created(
    State(app_state): State<AppState>,
    Extension(ctx): Extension<ReqContext>,
    Query(params): Query<schema::TokenCreatedQuery>,
) -> LibResult<impl IntoResponse> {
let rsp = logic::get_token_created(app_state, ctx.user_addr, params).await?;
    Ok(Resp200::new(rsp))
}