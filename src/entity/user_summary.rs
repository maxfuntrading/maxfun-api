use crate::utility::{LibError, LibResult};
use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;
use sea_orm::Statement;

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

impl Entity {
    pub async fn find_token_owned(
        db: &DatabaseConnection,
        user_address: String,
        keyword: Option<String>,
        page: u64,
        page_size: u64,
    ) -> LibResult<(Vec<(String, String, Decimal, Decimal)>, u64)> {
        // 先构建基础 SQL
        let base_sql = format!(
            r#"
            FROM
                user_summary t1
                LEFT JOIN token_info t2 ON t1.token_address = t2.token_address
                LEFT JOIN token_summary t3 ON t1.token_address = t3.token_address
            WHERE
                t1.user_address = '{}'
        "#,
            user_address
        );

        // 添加搜索条件
        let mut where_clause = String::new();
        if let Some(keyword) = keyword {
            where_clause = format!(
                r#"
                AND (
                    t2.token_address LIKE '%{}%'
                    OR t2.name LIKE '%{}%'
                    OR t2.symbol LIKE '%{}%'
                )
            "#,
                keyword, keyword, keyword
            );
        }

        // 计算总数
        let count_sql = format!(
            r#"
            SELECT COUNT(*) as total
            {}
            {}
        "#,
            base_sql, where_clause
        );
        
        let total: i64 = db
            .query_one(Statement::from_string(db.get_database_backend(), count_sql))
            .await?
            .ok_or_else(|| LibError::ParamError("Failed to get total count".to_string()))?
            .try_get("", "total")?;

        // 查询数据
        let mut sql = format!(
            r#"
            SELECT
                t2.icon,
                t2.symbol,
                t1.amount AS quantity,
                COALESCE(t3.price * t1.amount, 0)::decimal(40,18) as value
            {}
            {}
        "#,
            base_sql, where_clause
        );

        sql.push_str(" ORDER BY value DESC NULLS LAST");
        let offset = (page - 1) * page_size;
        sql.push_str(&format!(" LIMIT {} OFFSET {}", page_size, offset));

        let stmt = Statement::from_string(db.get_database_backend(), sql);
        let rows = db.query_all(stmt).await?;
        let tokens = rows
            .into_iter()
            .map(|row| {
                Ok((
                    row.try_get::<String>("", "icon")?,
                    row.try_get::<String>("", "symbol")?,
                    row.try_get::<Decimal>("", "quantity")?,
                    row.try_get::<Decimal>("", "value")?,
                ))
            })
            .collect::<Result<Vec<_>, DbErr>>()?;

        Ok((tokens, total as u64))
    }
}
