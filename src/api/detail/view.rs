use axum::{extract::State, response::IntoResponse, extract::Query};
use crate::core::AppState;
use crate::utility::{LibResult, Resp200};
use crate::api::detail::{schema, logic};

pub async fn basic_info(
    State(app_state): State<AppState>,
    Query(params): Query<schema::BasicInfoQuery>,
) -> LibResult<impl IntoResponse> {
    let rsp = logic::get_basic_info(&app_state, &params.token_address).await?;
    Ok(Resp200::new(rsp))
}

pub async fn kline(
    State(app_state): State<AppState>,
    Query(params): Query<schema::KlineQuery>,
) -> LibResult<impl IntoResponse> {
    let rsp = logic::get_kline(
        &app_state,
        &params.token_address,
        params.last_open_ts,
        params.limit,
    ).await?;
    Ok(Resp200::new(rsp))
}

pub async fn comment_history(State(app_state): State<AppState>) -> LibResult<impl IntoResponse> {
    Ok("")
}

pub async fn comment_submit(State(app_state): State<AppState>) -> LibResult<impl IntoResponse> {
    Ok("")
}

pub async fn trade_log(
    State(app_state): State<AppState>,
    Query(params): Query<schema::TradeLogQuery>,
) -> LibResult<impl IntoResponse> {
    let rsp = logic::get_trade_log(
        &app_state,
        &params.token_address,
        params.last_block_number,
        params.last_txn_index,
        params.last_log_index,
        params.limit,
    ).await?;
    Ok(Resp200::new(rsp))
}

pub async fn holder_distribution(State(app_state): State<AppState>) -> LibResult<impl IntoResponse> {
    Ok("")
}