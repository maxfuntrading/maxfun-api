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
    pub price: Option<Decimal>,
    pub price_rate24h: Option<Decimal>,
    pub market_cap: Option<Decimal>,
    pub liquidity: Option<Decimal>,
    pub volume24h: Option<Decimal>,
    pub total_supply: Option<Decimal>,
    pub description: String,
    pub tag: Option<String>,
    pub website: Option<String>,
    pub twitter: Option<String>,
    pub telegram: Option<String>,
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
pub struct TradeLogQuery {
    pub token_address: String,
    pub last_block_time: Option<i64>,     // 区块时间
    pub last_block_number: Option<i64>,    // 区块号
    pub last_txn_index: Option<i64>,       // 交易索引
    pub limit: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct TradeLogData {
    pub user_address: String,
    pub trace_type: i32,      // 1: buy, 2: sell
    pub token1_amount: Decimal,
    pub token2_amount: Decimal,
    pub block_time: i64,
    pub block_number: i64,    // 区块号
    pub txn_index: i64,       // 交易索引
    pub txn_hash: String,
}

#[derive(Debug, Serialize)]
pub struct TradeLogResp {
    pub list: Vec<TradeLogData>,
}
