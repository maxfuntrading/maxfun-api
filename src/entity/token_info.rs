use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "token_info")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub token_address: String,
    pub user_address: String, // Token Creator
    pub name: String,         // Token Full Name
    pub icon: String,
    pub symbol: String, // Token Symbol
    pub description: String,
    pub tag: String,
    pub website: String,
    pub twitter: String,
    pub telegram: String,
    pub total_supply: Decimal,
    pub raised_token: String,
    pub raised_amount: Decimal,
    pub sale_ratio: Decimal,
    pub reserved_ratio: Decimal,
    pub pool_ratio: Decimal,
    pub launch_ts: i64,    // Token Launch Time
    pub create_ts: i64,    // Token Creation Time
    pub is_launched: bool, // Whether Listed on Uniswap
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::token_summary::Entity")]
    TokenSummary,
}

impl Related<super::token_summary::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TokenSummary.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
