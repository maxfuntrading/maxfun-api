use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "raised_token")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub decimal: i32,
    pub icon: String,
    pub price: Decimal,
    pub create_ts: i64,
    pub oracle: String, // oracle price address
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
