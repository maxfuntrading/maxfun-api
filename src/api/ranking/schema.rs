use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RankingItem {
    pub rank: i32,                // 排名
    pub token_address: String,    // 代币地址
    pub name: String,            // 代币名称
    pub icon: String,            // 代币图标
    pub market_cap: Option<Decimal>,  // 市值
    pub bonding_curve: Option<Decimal>,  // Bonding曲线
    pub price_rate24h: Option<Decimal>,  // 24h涨跌幅
    pub volume_24h: Option<Decimal>,    // 24h交易量
}

#[derive(Debug, Serialize)]
pub struct RankingResp {
    pub list: Vec<RankingItem>,
}
