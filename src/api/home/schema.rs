use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct MarqueeItem {
    pub user_address: String,
    pub trade_type: i32, // 0: buy, 1: sell
    pub token_address: String,
    pub amount: Decimal,
    pub icon: String,
    pub symbol: String,
    pub tag: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MarqueeListResp {
    pub list: Vec<MarqueeItem>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortField {
    LaunchTs,
    Volume24h,
    MarketCap,
    LastTrade,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Deserialize)]
pub struct TokenListQuery {
    pub keyword: Option<String>,       // search keyword
    pub tag: Option<String>,           // tag filter
    pub is_launched: Option<bool>,     // status
    pub sort_by: Option<SortField>,    // sort field
    pub sort_order: Option<SortOrder>, // sort order
    pub page: Option<u64>,             // page number, start from 1
    pub page_size: Option<u64>,        // page size
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
pub struct TokenListResp {
    pub list: Vec<TokenInfo>,
    pub total: u64,
}
