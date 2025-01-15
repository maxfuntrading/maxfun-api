use sea_orm::entity::prelude::*;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user_summary")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_address: String,
    #[sea_orm(primary_key)]
    pub token_address: String,
    pub amount: Decimal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {} 