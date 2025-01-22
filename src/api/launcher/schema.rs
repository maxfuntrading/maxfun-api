use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LaunchTokenReq{
    pub name: String,// 代币全称
    pub icon: String,
    pub symbol: String, // 代币缩写
    pub description: String,
    pub raised_token: String,
    pub tag: Option<String>,
    pub website: Option<String>,
    pub twitter: Option<String>,
    pub telegram: Option<String>,
    pub total_supply: Option<Decimal>, // 代币总量
    pub raised_amount: Option<Decimal>,
    pub sale_ratio: Option<Decimal>,
    pub reserved_ratio: Option<Decimal>,
    pub pool_ratio: Option<Decimal>,
    pub launch_ts: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct LaunchTokenResp{
    pub id: i32,
    pub signature: String,
}

#[derive(Debug, Deserialize)]
pub struct GetRaisedTokenPriceQuery{
    pub raised_token: String,
}