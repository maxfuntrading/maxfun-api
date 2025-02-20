use crate::api::launcher::{logic, schema};
use crate::core::state::ReqContext;
use crate::core::{consts, AppState};
use crate::utility::{LibError, LibResult, Resp200};
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::{Extension, Json};
use rust_decimal::Decimal;
use url::Url;

fn validate_url(url: &Option<String>) -> LibResult<()> {
    if let Some(url) = url {
        Url::parse(url).map_err(|_| LibError::ParamError("Invalid URL".to_string()))?;
    }
    Ok(())
}

async fn validate_token_request(
    app_state: &AppState,
    req: &schema::LaunchTokenReq,
) -> LibResult<()> {
    // Validate Token Name
    if req.name.chars().count() > 20
        || !req
            .name
            .chars()
            .all(|c| c.is_alphanumeric() || c.is_whitespace())
    {
        return Err(LibError::ParamError(
            "Token name must contain at most 20 characters".to_string(),
        ));
    }

    // Validate Token Symbol
    if req.symbol.chars().count() > 10 || !req.symbol.chars().all(|c| c.is_alphanumeric()) {
        return Err(LibError::ParamError(
            "Token symbol must contain at most 10 characters".to_string(),
        ));
    }

    // Validate description length (character count)
    if req.description.chars().count() > 256 {
        return Err(LibError::ParamError(
            "Description must contain at most 256 characters".to_string(),
        ));
    }

    // Validate URLs
    validate_url(&req.website)?;
    validate_url(&req.twitter)?;
    validate_url(&req.telegram)?;

    // Validate Total Supply
    if let Some(total_supply) = req.total_supply {
        if total_supply < Decimal::new((*consts::MIN_TOKEN_TOTAL_SUPPLY).into(), 0) {
            return Err(LibError::ParamError(format!(
                "The minimum total supply needs to be greater than {:?}",
                consts::MIN_TOKEN_TOTAL_SUPPLY
            )));
        }
    }

    // Validate ratio sum
    let sale_ratio = req.sale_ratio.unwrap_or(Decimal::new(80, 0));
    if sale_ratio < Decimal::new(60, 0) {
        return Err(LibError::ParamError(
            "Sales ratio must be at least 60%".to_string(),
        ));
    }

    let reserved_ratio = req.reserved_ratio.unwrap_or_default();
    let pool_ratio = req.pool_ratio.unwrap_or_default();

    if sale_ratio + reserved_ratio + pool_ratio != Decimal::new(100, 0) {
        return Err(LibError::ParamError("Total ratio must be 100%".to_string()));
    }

    // Validate raised amount
    if let Some(raised_amount) = req.raised_amount {
        let price = logic::get_raised_token_price(app_state, &req.raised_token).await?;
        if raised_amount * price < Decimal::new((*consts::MIN_RAISED_AMOUNT_USD).into(), 0) {
            return Err(LibError::ParamError(format!(
                "The minimum amount needs to be greater than ${:?}",
                consts::MIN_RAISED_AMOUNT_USD
            )));
        }
    }

    Ok(())
}

pub async fn launch_token(
    State(app_state): State<AppState>,
    Extension(ctx): Extension<ReqContext>,
    Json(payload): Json<schema::LaunchTokenReq>,
) -> LibResult<impl IntoResponse> {
    // Parameter validation
    validate_token_request(&app_state, &payload).await?;

    // Call business logic
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
