use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "evt_txn_log")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub block_number: i64,
    #[sea_orm(primary_key)]
    pub txn_index: i64,
    #[sea_orm(primary_key)]
    pub log_index: i64,
    pub block_time: i64,
    pub txn_hash: String,
    pub address: String,
    pub topic_0: String,
    pub topic_1: String,
    pub topic_2: String,
    pub topic_3: String,
    pub data: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
