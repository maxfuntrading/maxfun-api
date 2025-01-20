use crate::api::launcher::schema;
use crate::core::{consts, AppState};
use crate::entity::{token_info, RaisedToken};
use crate::utility::{LibError, LibResult};
use chrono::Utc;
use ethers::{
    signers::{LocalWallet, Signer},
    utils::keccak256,
};
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, EntityTrait, NotSet, Set};

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
        id: NotSet,
        token_address: Set("".to_string()),
        user_address: Set(user_address.clone()),
        name: Set(req.name),
        icon: Set(req.icon),
        symbol: Set(req.symbol),
        description: Set(req.description),
        tag: Set(req.tag),
        website: Set(req.website),
        twitter: Set(req.twitter),
        telegram: Set(req.telegram),
        total_supply: Set(Some(req.total_supply.unwrap_or_else(|| 
            Decimal::new((*consts::DEFAULT_TOKEN_TOTAL_SUPPLY).into(), 0)
        ))),
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

    // 生成签名消息
    let message = ethers::abi::encode(&[
        ethers::abi::Token::Address(user_address.parse().unwrap()),
        ethers::abi::Token::Uint(token.id.into()),
        ethers::abi::Token::Uint((*consts::CHAIN_ID).into()),
    ]);
    
    let message_hash = keccak256(message);

    // 直接使用私钥创建钱包
    let wallet = consts::EOA_PRIVATE_KEY.parse::<LocalWallet>()?;
    
    let signature = wallet.sign_message(&message_hash).await?;
    
    Ok(schema::LaunchTokenResp {
        id: token.id.to_string(),
        signature: signature.to_string(),
    })
}
