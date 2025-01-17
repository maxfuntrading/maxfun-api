use axum::{Extension, Json};
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use crate::core::AppState;
use crate::core::state::ReqContext;
use crate::utility::{LibError, LibResult, Resp200};
use crate::api::launcher::{schema, logic};
use url::Url;
use rust_decimal::Decimal;

fn validate_url(url: &Option<String>) -> LibResult<()> {
    if let Some(url) = url {
        Url::parse(url).map_err(|_| LibError::ParamError("Invalid URL".to_string()))?;
    }
    Ok(())
}

async fn validate_token_request(app_state: &AppState, req: &schema::LaunchTokenReq) -> LibResult<()> {
    // 验证 Token Name
    if req.name.len() > 20 || !req.name.chars().all(|c| c.is_alphanumeric() || c.is_whitespace()) {
        return Err(LibError::ParamError("String must contain at most 20 character(s)".to_string()));
    }

    // 验证 Token Symbol
    if req.symbol.len() > 10 || !req.symbol.chars().all(|c| c.is_alphanumeric()) {
        return Err(LibError::ParamError("String must contain at most 10 character(s)".to_string()));
    }

    // 验证 Description
    if req.description.len() > 256 {
        return Err(LibError::ParamError("String must contain at most 256 character(s)".to_string()));
    }

    // 验证 URLs
    validate_url(&req.website)?;
    validate_url(&req.twitter)?;
    validate_url(&req.telegram)?;

    // 验证 Total Supply
    if let Some(total_supply) = req.total_supply {
        if total_supply < Decimal::new(1_000_000, 0) {
            return Err(LibError::ParamError("The minimum amount needs to be greater than 1,000,000".to_string()));
        }
    }

    // 验证比例总和
    let sale_ratio = req.sale_ratio.unwrap_or(Decimal::new(80, 0));
    if sale_ratio < Decimal::new(60, 0) {
        return Err(LibError::ParamError("Sales ratio must be at least 60%".to_string()));
    }

    let reserved_ratio = req.reserved_ratio.unwrap_or_default();
    let pool_ratio = req.pool_ratio.unwrap_or_default();
    
    if sale_ratio + reserved_ratio + pool_ratio != Decimal::new(100, 0) {
        return Err(LibError::ParamError("Total ratio must be 100%".to_string()));
    }

    // 验证 raised amount
    if let Some(raised_amount) = req.raised_amount {
        let price = logic::get_raised_token_price(app_state, &req.raised_token).await?;
        if raised_amount * price < Decimal::new(2000, 0) {
            return Err(LibError::ParamError("The minimum amount needs to be greater than $2,000".to_string()));
        }
    }

    Ok(())
}

pub async fn launch_token(
    State(app_state): State<AppState>,
    Extension(ctx): Extension<ReqContext>,
    Json(payload): Json<schema::LaunchTokenReq>,
) -> LibResult<impl IntoResponse> {
    // 参数校验
    validate_token_request(&app_state, &payload).await?;
    
    // 调用业务逻辑
    let rsp = logic::launch_token(app_state, ctx.user_addr, payload).await?;
    Ok(Resp200::new(rsp))
}

pub async fn get_raised_token_price(
    State(app_state): State<AppState>,
    Query(params): Query<schema::GetRaisedTokenPriceQuery>,
) -> LibResult<impl IntoResponse> {
    let rsp = logic::get_raised_token_price(&app_state, &params.raised_token).await?;
    Ok(Resp200::new(rsp))
}