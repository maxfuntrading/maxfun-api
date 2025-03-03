use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize)]
pub struct BasicInfoQuery {
    pub token_address: String,
}

#[derive(Debug, Serialize)]
pub struct TokenBasicInfo {
    pub name: String,
    pub symbol: String,
    pub icon: String,
    pub description: String,
    pub tag: String,
    pub website: String,
    pub twitter: String,
    pub telegram: String,
    pub total_supply: Decimal,
    pub price: Decimal,
    pub price_token: Decimal,
    pub price_rate24h: Decimal,
    pub market_cap: Decimal,
    pub liquidity: Decimal,
    pub volume24h: Decimal,
}

#[derive(Debug, Serialize)]
pub struct RaisedTokenInfo {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub icon: String,
    pub decimal: i32,
}

#[derive(Debug, Serialize)]
pub struct BasicInfoResp {
    pub token_basic: TokenBasicInfo,
    pub raised_token: RaisedTokenInfo,
}

#[derive(Debug, Deserialize)]
pub struct KlineQuery {
    pub token_address: String,
    pub last_open_ts: Option<i64>,
    pub limit: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct KlineData {
    pub open_ts: i64,
    pub close_ts: i64,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
    pub amount: Decimal,
    pub txn_num: i64,
}

#[derive(Debug, Serialize)]
pub struct KlineResp {
    pub list: Vec<KlineData>,
}

#[derive(Debug, Deserialize)]
pub struct CommentHistoryQuery {
    pub token_address: String,
    pub page: Option<u64>,      // Page number, starts from 1
    pub page_size: Option<u64>, // Items per page
}

#[derive(Debug, Serialize)]
pub struct CommentHistoryData {
    pub id: i32,
    pub user_address: String,
    pub user_avatar: String,
    pub comment: String,
    pub create_ts: i64,
}

#[derive(Debug, Serialize)]
pub struct CommentHistoryResp {
    pub list: Vec<CommentHistoryData>,
    pub total: u64, // Total record count
}

#[derive(Debug, Deserialize)]
pub struct CommentSubmitReq {
    pub token_address: String,
    pub comment: String,
}

#[derive(Debug, Deserialize)]
pub struct TradeLogQuery {
    pub token_address: String,
    pub last_block_number: Option<i64>,
    pub last_txn_index: Option<i64>,
    pub last_log_index: Option<i64>,
    pub limit: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct TradeLogData {
    pub block_number: i64,
    pub txn_index: i64,
    pub log_index: i64,
    pub user_address: String,
    pub trade_type: i32,        // 1: buy, 2: sell
    pub token1_amount: Decimal, // agent token amount
    pub token2_amount: Decimal, // raised token amount
    pub block_time: i64,
    pub txn_hash: String,
    pub price: Decimal,
    pub price_token: Decimal,
}

#[derive(Debug, Serialize)]
pub struct TradeLogResp {
    pub token1_name: String, // agent token name
    pub token2_name: String, // raised token name
    pub list: Vec<TradeLogData>,
}

#[derive(Debug, Deserialize)]
pub struct HolderDistributionQuery {
    pub token_address: String,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct HolderData {
    pub user_address: String, // Wallet address
    pub amount: Decimal,      // Holding amount
    pub percentage: Decimal,  // Holding percentage
}

#[derive(Debug, Serialize)]
pub struct HolderDistributionResp {
    pub total_holders: u64,    // Total number of holders
    pub list: Vec<HolderData>, // List of holders
}
