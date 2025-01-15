use sea_orm::entity::prelude::*;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "token_info")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub token_address: String,
    pub user_address: Option<String>,
    pub name: String,
    pub icon: Option<String>,
    pub symbol: String,
    pub description: Option<String>,
    pub tag: String,
    pub website: Option<String>,
    pub twitter: Option<String>,
    pub telegram: Option<String>,
    pub total_supply: Option<Decimal>,
    pub raised_token: Option<String>,
    pub raised_amount: Option<Decimal>,
    pub sale_ratio: Option<Decimal>,
    pub reserved_ratio: Option<Decimal>,
    pub pool_ratio: Option<Decimal>,
    pub launch_ts: Option<i64>,
    pub maxbuy_amount: Option<Decimal>,
    pub create_ts: i64,
    pub is_launched: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {} 