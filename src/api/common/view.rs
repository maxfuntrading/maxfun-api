use axum::extract::State;
use axum::response::IntoResponse;
use crate::api::common::logic;
use crate::core::AppState;
use crate::utility::{LibResult, Resp200};

pub async fn get_tags(State(app_state): State<AppState>) -> LibResult<impl IntoResponse> {
    let rsp = logic::get_tags(app_state).await?;
    Ok(Resp200::new(rsp))
}