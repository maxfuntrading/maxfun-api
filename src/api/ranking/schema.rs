use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RankingItem {
    pub rank: i32,             // Rank
    pub token_address: String, // Token address
    pub name: String,          // Token name
    pub symbol: String,
    pub icon: String,           // Token icon
    pub market_cap: Decimal,    // Market cap
    pub bonding_curve: Decimal, // Bonding curve
    pub price_rate24h: Decimal, // 24h price change
    pub volume_24h: Decimal,    // 24h trading volume
}

#[derive(Debug, Serialize)]
pub struct RankingResp {
    pub ranking_update_ts: i64,
    pub list: Vec<RankingItem>,
}
