use ethers::prelude::{Http, Provider};
use thiserror::Error;
use ethers::signers::WalletError;

#[derive(Error, Debug)]
pub enum LibError {
    #[error("need environment variable: {0}")]
    BadEnv(#[from] std::env::VarError),

    #[error("{0}")]
    SeaOrmError(#[from] sea_orm::DbErr),

    #[error("format error: {0}")]
    FormatError(#[from] std::fmt::Error),

    #[error("parse int error: {0}")]
    ParseError(#[from] std::num::ParseIntError),

    #[error("redis error: {0}")]
    RedisError(#[from] redis::RedisError),

    #[error("ethers error: {0}")]
    SignerError(#[from] ethers::core::types::SignatureError),

    #[error("ethers error: {0}")]
    EthersError(#[from] ethers::providers::ProviderError),

    #[error("chain error: {0}")]
    ChainError(#[from] ethers::contract::ContractError<Provider<Http>>),

    #[error("JWT token error: {0}")]
    JWTokenErr(#[from] jsonwebtoken::errors::Error),

    #[error("abi error: {0}")]
    AbiError(#[from] ethers::abi::Error),

    #[error("params error: {0}")]
    ParamError(String),

    #[error("request error: {0}")]
    ReqwestErr(#[from] reqwest::Error),

    #[error("serde_json error: {0}")]
    SerdeJsonErr(#[from] serde_json::Error),

    #[error("siwe message invalid")]
    SiweMessageInvalid,

    #[error("siwe signature invalid")]
    SiweSignInvalid,

    #[error("cookie invalid")]
    CookieInvalid,

    #[error("other error: {0}")]
    Other(#[from] anyhow::Error),

    #[error("User Not Found")]
    UserNotFound,

    #[error("Upload failed")]
    UploadFailed,

    #[error("File too large")]
    FileTooLarge,

    #[error("Invalid file type")]
    InvalidFileType,

    #[error("No file uploaded")]
    NoFileUploaded,

    #[error("wallet error: {0}")]
    WalletError(#[from] WalletError),
}
