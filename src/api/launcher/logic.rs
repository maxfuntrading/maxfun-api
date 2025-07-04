use crate::api::launcher::schema;
use crate::core::{consts, AppState};
use crate::entity::{token_info, RaisedToken};
use crate::utility::{LibError, LibResult};
use chrono::Utc;
use ethers::prelude::*;
use ethers::utils::keccak256;
use hex;
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, EntityTrait, NotSet, Set};

pub async fn get_raised_token_price(
    app_state: &AppState,
    token_address: &str,
) -> LibResult<Decimal> {
    let raised_token = RaisedToken::find_by_id(token_address.to_lowercase())
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
    // Create token
    let token = token_info::ActiveModel {
        id: NotSet,
        token_address: Set("".to_string()),
        user_address: Set(user_address.clone()),
        name: Set(req.name),
        icon: Set(req.icon),
        symbol: Set(req.symbol),
        description: Set(req.description),
        tag: set_option(req.tag),
        website: set_option(req.website),
        twitter: set_option(req.twitter),
        telegram: set_option(req.telegram),
        total_supply: Set(req
            .total_supply
            .unwrap_or_else(|| Decimal::new((*consts::DEFAULT_TOKEN_TOTAL_SUPPLY).into(), 0))),
        raised_token: Set(req.raised_token.to_lowercase()),
        raised_amount: set_option(req.raised_amount),
        sale_ratio: set_option(req.sale_ratio),
        reserved_ratio: set_option(req.reserved_ratio),
        pool_ratio: set_option(req.pool_ratio),
        launch_ts: set_option(req.launch_ts),
        create_ts: Set(Utc::now().timestamp()),
        is_launched: Set(false),
    };

    let token = token.insert(&app_state.db_pool).await?;
    let id_padded_hex = format!("{:0>64}", format!("{:016X}", token.id));

    // Get current chain id
    let chain_id_padded_hex = format!("{:0>64}", format!("{:016X}", app_state.chain_id));

    // Generate signature message
    let message = abi::encode_packed(&[
        abi::Token::Address(user_address.parse().unwrap()),
        abi::Token::Bytes(hex::decode(id_padded_hex).unwrap().into()),
        abi::Token::Bytes(hex::decode(chain_id_padded_hex).unwrap().into()),
    ])
    .map_err(|e| LibError::ParamError(e.to_string()))?;
    let message_hash = keccak256(message);
    // Directly use private key to create wallet
    let wallet = consts::EOA_PRIVATE_KEY.parse::<LocalWallet>()?;
    // Sign message
    let signature = wallet.sign_message(&message_hash).await?;
    Ok(schema::LaunchTokenResp {
        id: token.id,
        signature: format!("0x{}", signature.to_string()),
    })
}

fn set_option<T: Into<sea_orm::Value>>(value: Option<T>) -> sea_orm::ActiveValue<T> {
    match value {
        Some(v) => Set(v),
        None => NotSet,
    }
}
