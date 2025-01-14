use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Char(Some(42))")]
    pub address: String,
    #[sea_orm(column_type = "String(Some(255))")]
    pub name: String,
    #[sea_orm(column_type = "String(Some(255))", nullable)]
    pub avatar: Option<String>,
    pub create_ts: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {} 