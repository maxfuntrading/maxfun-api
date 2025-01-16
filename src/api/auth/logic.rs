use sea_orm::{EntityTrait, ActiveModelTrait, Set};
use siwe::{generate_nonce, Message, VerificationOpts};
use ethers::utils::hex;
use ethers::utils::hex::FromHex;
use chrono::Utc;

use super::schema;
use crate::core::state::AppState;
use crate::utility::*;
use crate::entity::User;
use crate::entity::UserAvatar;
use crate::core::consts;

pub async fn nonce() -> LibResult<schema::NonceRsp> {
    Ok(schema::NonceRsp {
        nonce: generate_nonce(),
    })
}

pub async fn verify(
    app_state: AppState,
    payload: schema::VerifyReq,
    nonce: String,
) -> LibResult<schema::VerifyResp> {
    // siwe verify the payload message and signature
    let message: Message = payload.message.parse().map_err(|_| LibError::SiweMessageInvalid)?;
    let address = format!("0x{}", hex::encode(message.address)).to_lowercase();
    tracing::info!("address {}", address);

    {
        let mut signature_str = payload.signature;
        if signature_str.starts_with("0x") {
            signature_str = signature_str[2..].to_string();
        }
        let signature = <[u8; 65]>::from_hex(signature_str).map_err(|_| LibError::SiweSignInvalid)?;

        let verification_opts = VerificationOpts {
            nonce: Some(nonce),
            ..Default::default()
        };
        message
            .verify(&signature, &verification_opts)
            .await
            .map_err(|_| LibError::SiweSignInvalid)?;
    }

    let user = User::find_by_id(&address).one(&app_state.db_pool).await?;

    if user.is_none() {
        let name = format!("{}...{}", &address[..6], &address[address.len()-6..]);
        
        let avatar = UserAvatar::get_random_avatar(&app_state.db_pool).await?;
        
        let new_user = crate::entity::user::ActiveModel {
            address: Set(address.clone()),
            name: Set(name),
            avatar: Set(avatar),
            create_ts: Set(Utc::now().timestamp()),
        };
        new_user.insert(&app_state.db_pool).await?;
    }

    Ok(schema::VerifyResp {
        user_exists: user.is_some(),
        auth_type: "Bearer".into(),
        auth_token: jwt::encode_token(address)?,
    })
}
