use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct UserInfoResp {
    pub address: String,
    pub name: String,
    pub avatar: String,
    pub create_ts: i64,
}

#[derive(Deserialize, Debug)]
pub struct TokenOwnedQuery {
    pub keyword: Option<String>, // search keyword
    pub page: Option<u64>,       // page number, start from 1
    pub page_size: Option<u64>,  // page size
}

#[derive(Serialize, Debug)]
pub struct TokenOwned {
    pub token_address: String,
    pub token_icon: String,
    pub token_symbol: String,
    pub quantity: Decimal,
    pub value: Decimal,
}

#[derive(Serialize, Debug)]
pub struct TokenOwnedResp {
    pub list: Vec<TokenOwned>,
    pub total: u64,
}

#[derive(Debug, Deserialize)]
pub struct TokenCreatedQuery {
    pub keyword: Option<String>, // search keyword
    pub page: Option<u64>,       // page number, start from 1
    pub page_size: Option<u64>,  // page size
}

#[derive(Debug, Serialize)]
pub struct TokenInfo {
    pub token_address: String,
    pub icon: String,
    pub tag: String,
    pub user_address: String, // Create By
    pub name: String,         // Token Name
    pub symbol: String,       // Token Symbol
    pub description: String,
    pub market_cap: Decimal,
    pub bonding_curve: Decimal,
    pub price_rate24h: Decimal,
    pub is_launched: bool, // Listed on Uniswap
}

#[derive(Debug, Serialize)]
pub struct TokenCreatedResp {
    pub list: Vec<TokenInfo>,
    pub total: u64,
}
