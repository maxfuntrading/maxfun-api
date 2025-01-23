use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize)]
pub struct BasicInfoQuery {
    pub token_address: String,
}

#[derive(Debug, Serialize)]
pub struct BasicInfoResp {
    pub name: String,
    pub symbol: String,
    pub icon: String,
    pub price: Decimal,
    pub price_rate24h: Decimal,
    pub market_cap: Decimal,
    pub liquidity: Decimal,
    pub volume24h: Decimal,
    pub total_supply: Decimal,
    pub description: String,
    pub tag: String,
    pub website: String,
    pub twitter: String,
    pub telegram: String,
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
    pub page: Option<u64>,      // 页码，从1开始
    pub page_size: Option<u64>, // 每页大小
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
    pub total: u64, // 总记录数
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
    pub trace_type: i32,        // 1: buy, 2: sell
    pub token1_amount: Decimal, // agent token amount
    pub token2_amount: Decimal, // raised token amount
    pub block_time: i64,
    pub txn_hash: String,
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
    pub user_address: String, // 钱包地址
    pub amount: Decimal,      // 持有数量
    pub percentage: Decimal,  // 持有占比
}

#[derive(Debug, Serialize)]
pub struct HolderDistributionResp {
    pub total_holders: u64,    // 持有者总数
    pub list: Vec<HolderData>, // 持有者列表
}
