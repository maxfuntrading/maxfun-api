use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "token_summary")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub token_address: String,
    pub raised_token: String,
    pub price: Decimal,
    pub price_rate24h: Decimal,
    pub volume_24h: Decimal,
    pub total_supply: Decimal,
    pub market_cap: Decimal,
    pub bonding_curve: Decimal,
    pub uniswap_pool: String,
    pub last_trade_ts: i64,
    pub price_token: Decimal,
    pub pair_address: String,
    pub liquidity: Decimal,       // Meme token sell amount * price
    pub liquidity_token: Decimal, // Meme token sell amount
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::token_info::Entity",
        from = "Column::TokenAddress",
        to = "super::token_info::Column::TokenAddress"
    )]
    TokenInfo,
}

impl Related<super::token_info::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TokenInfo.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
