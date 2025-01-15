use crate::api::home::{logic, schema};
use crate::core::AppState;
use crate::utility::{LibResult, Resp200};
use axum::extract::{State, Query};
use axum::response::IntoResponse;

pub async fn get_marquee(
    State(app_state): State<AppState>,
) -> LibResult<impl IntoResponse> {
    let rsp = logic::get_marquee(app_state).await?;
    Ok(Resp200::new(rsp))
}

pub async fn get_token_list(
    State(app_state): State<AppState>,
    Query(query): Query<schema::TokenListQuery>,
) -> LibResult<impl IntoResponse> {
    let rsp = logic::get_token_list(app_state, query).await?;
    Ok(Resp200::new(rsp))
}

pub async fn get_token_tags(State(app_state): State<AppState>) -> LibResult<impl IntoResponse> {
    let rsp = logic::get_token_tags(app_state).await?;
    Ok(Resp200::new(rsp))
}