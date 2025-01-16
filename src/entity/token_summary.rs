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
    pub last_trade_ts: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::token_info::Entity", from = "Column::TokenAddress", to = "super::token_info::Column::TokenAddress")]
    TokenInfo,
}

impl Related<super::token_info::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TokenInfo.def()
    }
}

impl ActiveModelBehavior for ActiveModel {} 