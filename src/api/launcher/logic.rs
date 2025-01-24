use crate::api::launcher::schema;
use crate::core::{consts, AppState};
use crate::entity::{token_info, RaisedToken};
use crate::utility::{LibError, LibResult};
use chrono::Utc;
use ethers::core::types::{Address, U256};
use ethers::prelude::*;
use ethers::utils::keccak256;
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

    // 准备消息内容
    let address: Address = user_address.parse()
        .map_err(|e| LibError::ParamError(format!("Invalid address: {}", e)))?;
    let id: u8 = token.id as u8;
    let chain_id = U256::from(*consts::CHAIN_ID);

    // Solidity packed keccak256 计算
    let types = vec!["address", "uint8", "uint256"];
    let mut chain_id_bytes = [0u8; 32];
    chain_id.to_big_endian(&mut chain_id_bytes);
    
    let values: Vec<Vec<u8>> = vec![
        address.as_bytes().to_vec(),
        vec![id],
        chain_id_bytes.to_vec(),
    ];
    
    let packed = solidity_packed(types, values);
    let message_hash = keccak256(packed);
    println!("\n=== Message Hash ===");
    println!("Hash (hex): 0x{}", hex::encode(&message_hash));

    // 直接使用私钥创建钱包
    let wallet = consts::EOA_PRIVATE_KEY.parse::<LocalWallet>()?;

    // 签名消息
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

// 打包数据，根据 Solidity 的打包规则
fn solidity_packed(types: Vec<&str>, values: Vec<Vec<u8>>) -> Vec<u8> {
    let mut packed = Vec::new();
    for (ty, value) in types.iter().zip(values.iter()) {
        match *ty {
            "address" => {
                // Solidity 地址类型是 32 字节，前面用零填充
                let mut addr = vec![0u8; 32];
                addr[12..].copy_from_slice(&value);
                packed.extend_from_slice(&addr);
            }
            "uint8" => {
                // uint8 是一个字节
                packed.push(value[0]);
            }
            "uint256" => {
                // uint256 是 32 字节
                packed.extend_from_slice(value);
            }
            _ => panic!("不支持的类型: {}", ty),
        }
    }
    packed
}