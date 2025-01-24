use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RankingItem {
    pub rank: i32,             // 排名
    pub token_address: String, // 代币地址
    pub name: String,          // 代币名称
    pub symbol: String,
    pub icon: String,           // 代币图标
    pub market_cap: Decimal,    // 市值
    pub bonding_curve: Decimal, // Bonding曲线
    pub price_rate24h: Decimal, // 24h涨跌幅
    pub volume_24h: Decimal,    // 24h交易量
}

#[derive(Debug, Serialize)]
pub struct RankingResp {
    pub ranking_update_ts: i64,
    pub list: Vec<RankingItem>,
}
