use crate::api::detail::{logic, schema};
use crate::core::state::ReqContext;
use crate::core::AppState;
use crate::utility::{LibError, LibResult, Resp200};
use axum::{extract::Query, extract::State, response::IntoResponse, Extension, Json};

pub async fn basic_info(
    State(app_state): State<AppState>,
    Query(params): Query<schema::BasicInfoQuery>,
) -> LibResult<impl IntoResponse> {
    let rsp = logic::get_basic_info(app_state, &params.token_address).await?;
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
    )
    .await?;
    Ok(Resp200::new(rsp))
}

pub async fn comment_history(
    State(app_state): State<AppState>,
    Query(params): Query<schema::CommentHistoryQuery>,
) -> LibResult<impl IntoResponse> {
    let rsp = logic::comment_history(
        app_state,
        &params.token_address,
        params.page,
        params.page_size,
    )
    .await?;
    Ok(Resp200::new(rsp))
}

pub async fn comment_submit(
    State(app_state): State<AppState>,
    Extension(ctx): Extension<ReqContext>,
    Json(payload): Json<schema::CommentSubmitReq>,
) -> LibResult<impl IntoResponse> {
    // 验证评论长度
    let comment_len = payload.comment.chars().count();
    if comment_len < 1 || comment_len > 256 {
        return Err(LibError::ParamError(
            "Comment length must be between 1 and 256 characters".to_string(),
        ));
    }

    // 验证评论内容
    if payload.comment.trim().is_empty() {
        return Err(LibError::ParamError(
            "Comment cannot be empty or only whitespace".to_string(),
        ));
    }

    let rsp = logic::comment_submit(
        app_state,
        ctx.user_addr,
        payload.token_address,
        payload.comment,
    )
    .await?;
    Ok(Resp200::new(rsp))
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
    )
    .await?;
    Ok(Resp200::new(rsp))
}

pub async fn holder_distribution(
    State(app_state): State<AppState>,
    Query(params): Query<schema::HolderDistributionQuery>,
) -> LibResult<impl IntoResponse> {
    let rsp = logic::holder_distribution(
        app_state,
        &params.token_address,
        params.page,
        params.page_size,
    )
    .await?;
    Ok(Resp200::new(rsp))
}
