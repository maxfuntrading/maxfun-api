use crate::api::ranking::logic;
use crate::core::AppState;
use crate::utility::{LibResult, Resp200};
use axum::{extract::State, response::IntoResponse};

pub async fn process_ranking(
    State(app_state): State<AppState>,
) -> LibResult<impl IntoResponse> {
    let rsp = logic::get_process_ranking(app_state).await?;
    Ok(Resp200::new(rsp))
}

pub async fn gainer_ranking(
    State(app_state): State<AppState>,
) -> LibResult<impl IntoResponse> {
    let rsp = logic::get_gainer_ranking(app_state).await?;
    Ok(Resp200::new(rsp))
}

pub async fn market_cap_ranking(
    State(app_state): State<AppState>,
) -> LibResult<impl IntoResponse> {
    let rsp = logic::get_market_cap_ranking(app_state).await?;
    Ok(Resp200::new(rsp))
}

pub async fn trading_volume_ranking(
    State(app_state): State<AppState>,
) -> LibResult<impl IntoResponse> {
    let rsp = logic::get_volume_ranking(app_state).await?;
    Ok(Resp200::new(rsp))
}
