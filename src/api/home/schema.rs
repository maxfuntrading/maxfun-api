use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Serialize)]
pub struct TokenTag {
    pub name: String,
    pub sort: i32,
}

#[derive(Debug, Serialize)]
pub struct TokenTagListResp {
    pub list: Vec<TokenTag>,
}

#[derive(Debug, Serialize)]
pub struct MarqueeItem {
    pub user_address: String,
    pub trade_type: i32,  // 0: buy, 1: sell
    pub token_address: String,
    pub amount: Decimal,
    pub icon: Option<String>,
    pub symbol: String,
    pub tag: String,
}

#[derive(Debug, Serialize)]
pub struct MarqueeListResp {
    pub list: Vec<MarqueeItem>,
}

#[derive(Debug, Deserialize)]
pub struct TokenListQuery {
    pub keyword: Option<String>,     // search keyword
    pub tag: Option<String>,         // tag filter
    pub is_launched: Option<bool>,   // status
}

#[derive(Debug, Serialize)]
pub struct TokenInfo {
    pub token_address: String,
    pub user_address: Option<String>,
    pub name: String,
    pub icon: Option<String>,
    pub symbol: String,
    pub description: Option<String>,
    pub tag: String,
    pub website: Option<String>,
    pub twitter: Option<String>,
    pub telegram: Option<String>,
    pub total_supply: Option<Decimal>,
    pub raised_token: Option<String>,
    pub raised_amount: Option<Decimal>,
    pub sale_ratio: Option<Decimal>,
    pub reserved_ratio: Option<Decimal>,
    pub pool_ratio: Option<Decimal>,
    pub launch_ts: Option<i64>,
    pub maxbuy_amount: Option<Decimal>,
    pub create_ts: i64,
    pub is_launched: bool,
}

#[derive(Debug, Serialize)]
pub struct TokenListResp {
    pub list: Vec<TokenInfo>,
}