use crate::api::launcher::schema;
use crate::core::{consts, AppState};
use crate::entity::{token_info, RaisedToken};
use crate::utility::{LibError, LibResult};
use chrono::Utc;
use ethers::{
    abi::{encode_packed, Token},
    signers::{LocalWallet, Signer},
    utils::keccak256,
};
use hex;
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, EntityTrait, NotSet, Set};

pub async fn get_raised_token_price(
    app_state: &AppState,
    token_address: &str,
) -> LibResult<Decimal> {
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
        tag: set_option(req.tag),
        website: set_option(req.website),
        twitter: set_option(req.twitter),
        telegram: set_option(req.telegram),
        total_supply: Set(req
            .total_supply
            .unwrap_or_else(|| Decimal::new((*consts::DEFAULT_TOKEN_TOTAL_SUPPLY).into(), 0))),
        raised_token: Set(req.raised_token),
        raised_amount: set_option(req.raised_amount),
        sale_ratio: set_option(req.sale_ratio),
        reserved_ratio: set_option(req.reserved_ratio),
        pool_ratio: set_option(req.pool_ratio),
        launch_ts: set_option(req.launch_ts),
        create_ts: Set(Utc::now().timestamp()),
        is_launched: Set(false),
    };

    let token = token.insert(&app_state.db_pool).await?;

    println!("\n=== Signature Parameters ===");
    println!("User Address: {}", user_address);
    println!("Token ID: {}", token.id);
    println!("Chain ID: {}", *consts::CHAIN_ID);

    // 使用 encode_packed 和 keccak256 模拟 solidityPackedKeccak256
    let message = encode_packed(&[
        Token::Address(user_address.parse().unwrap()),
        Token::Uint(token.id.into()),
        Token::Uint((*consts::CHAIN_ID).into()),
    ])
    .map_err(|e| LibError::ParamError(e.to_string()))?;

    println!("\n=== Encoded Message ===");
    println!("Packed message (hex): 0x{}", hex::encode(&message));

    let message_hash = keccak256(message);
    println!("\n=== Message Hash ===");
    println!("Hash (hex): 0x{}", hex::encode(&message_hash));

    // 直接使用私钥创建钱包
    let wallet = consts::EOA_PRIVATE_KEY.parse::<LocalWallet>()?;

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
