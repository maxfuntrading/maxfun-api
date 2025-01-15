use sea_orm::entity::prelude::*;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "token_summary")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub token_address: String,
    pub raised_token: Option<String>,
    pub price: Option<Decimal>,
    pub price_rate24h: Option<Decimal>,
    pub volume_rate24h: Option<Decimal>,
    pub liquidity: Option<Decimal>,
    pub total_supply: Option<Decimal>,
    pub market_cap: Option<Decimal>,
    pub uniswap_pool: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {} 