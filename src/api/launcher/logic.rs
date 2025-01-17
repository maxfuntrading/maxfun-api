use crate::core::AppState;
use crate::api::launcher::schema;
use crate::utility::{LibError, LibResult};
use crate::entity::{token_info, RaisedToken};
use sea_orm::{ActiveModelTrait, Set, EntityTrait};
use chrono::Utc;
use rust_decimal::Decimal;

pub async fn get_raised_token_price(app_state: &AppState, token_address: &str) -> LibResult<Decimal> {
    let raised_token = RaisedToken::find_by_id(token_address)
        .one(&app_state.db_pool)
        .await?
        .ok_or_else(|| LibError::ParamError("Invalid raised token".to_string()))?;
    
    Ok(raised_token.price)
}

pub async fn launch_token(
    app_state: AppState,
    user_address: String,
    req: schema::LaunchTokenReq,
) -> LibResult<schema::LaunchTokenResp> {
    // 创建代币
    let token = token_info::ActiveModel {
        id: Set(Default::default()),
        token_address: Set("".to_string()),
        user_address: Set(user_address),
        name: Set(req.name),
        icon: Set(req.icon),
        symbol: Set(req.symbol),
        description: Set(req.description),
        tag: Set(req.tag),
        website: Set(req.website),
        twitter: Set(req.twitter),
        telegram: Set(req.telegram),
        total_supply: Set(req.total_supply),
        raised_token: Set(req.raised_token),
        raised_amount: Set(req.raised_amount),
        sale_ratio: Set(req.sale_ratio),
        reserved_ratio: Set(req.reserved_ratio),
        pool_ratio: Set(req.pool_ratio),
        launch_ts: Set(req.launch_ts),
        create_ts: Set(Utc::now().timestamp()),
        is_launched: Set(false),
    };

    let token = token.insert(&app_state.db_pool).await?;

    Ok(schema::LaunchTokenResp {
        id: token.id.to_string(),
        signature: "".to_string(),  // todo 待实现签名逻辑
    })
}