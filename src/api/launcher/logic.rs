use crate::api::launcher::schema;
use crate::core::{consts, AppState};
use crate::entity::{token_info, RaisedToken};
use crate::utility::{LibError, LibResult};
use chrono::Utc;
use ethers::{
    signers::{LocalWallet, Signer, Wallet},
    utils::keccak256,
};
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, EntityTrait, NotSet, Set};
use std::{path::PathBuf, fs};

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

    // 生成签名消息
    let message = ethers::abi::encode(&[
        ethers::abi::Token::Address(user_address.parse().unwrap()),
        ethers::abi::Token::Uint(token.id.into()),
        ethers::abi::Token::Uint((*consts::CHAIN_ID).into()),
    ]);
    
    let message_hash = keccak256(message);

    // 获取钱包并签名
    let wallet = get_wallet(consts::CREATOR_ADDRESS.as_str(), consts::CREATOR_PASSWORD.as_str()).await?;
    
    let signature = wallet.sign_message(&message_hash).await?;
    
    Ok(schema::LaunchTokenResp {
        id: token.id.to_string(),
        signature: signature.to_string(),
    })
}

async fn get_wallet(address: &str, password: &str) -> LibResult<LocalWallet> {
    let keystore_path = PathBuf::from(consts::KEYSTORE_DIR.as_str());
    // 处理地址格式
    let clean_address = address.trim_start_matches("0x").to_lowercase();
    // 读取目录下的所有文件
    let entries = fs::read_dir(&keystore_path)
    .map_err(|e| LibError::ParamError(format!("Failed to read keystore directory: {}", e)))?;
    // 查找匹配地址的 keystore 文件
    let keystore_file = entries
        .filter_map(Result::ok)
        .find(|entry| {
            entry.file_name()
                .to_string_lossy()
                .to_lowercase()
                .ends_with(&clean_address)
        })
        .ok_or_else(|| LibError::ParamError("Keystore file not found".to_string()))?;
    // 使用 ethers 内置的 Wallet::decrypt_keystore
    let wallet = Wallet::decrypt_keystore(keystore_file.path(), password)
        .map_err(|e| LibError::ParamError(format!("Failed to decrypt keystore: {}", e)))?;

    Ok(wallet)
}