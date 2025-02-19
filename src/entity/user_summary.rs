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
    ) -> LibResult<(Vec<(String, String, String, Decimal, Decimal)>, u64)> {
        // 先构建基础 SQL
        let base_sql = r#"
            FROM
                user_summary t1
                LEFT JOIN token_info t2 ON t1.token_address = t2.token_address
                LEFT JOIN token_summary t3 ON t1.token_address = t3.token_address
            WHERE
                t1.user_address = $1
        "#;

        // 添加搜索条件
        let mut where_clause = String::new();
        let mut params: Vec<sea_orm::Value> = vec![user_address.clone().into()];
        
        if let Some(keyword) = keyword {
            // 检查是否是有效的以太坊地址格式（0x开头的42位十六进制）
            let is_eth_address = keyword.len() == 42 
                && keyword.starts_with("0x") 
                && keyword[2..].chars().all(|c| c.is_ascii_hexdigit());

            if is_eth_address {
                // token_address 全匹配，不区分大小写
                where_clause = " AND t2.token_address ILIKE $2".to_string();
                params.push(keyword.into());
            } else {
                // 其他字段模糊匹配
                where_clause = r#" 
                    AND (
                        t2.name ILIKE $2
                        OR t2.symbol ILIKE $2
                    )
                "#.to_string();
                params.push(format!("%{}%", keyword).into());
            }
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
        
        let stmt = Statement::from_sql_and_values(db.get_database_backend(), &count_sql, params.clone());
        
        let total: i64 = db
            .query_one(stmt)
            .await?
            .ok_or_else(|| LibError::ParamError("Failed to get total count".to_string()))?
            .try_get("", "total")?;

        // 查询数据
        let mut sql = format!(
            r#"
            SELECT
                t2.token_address,
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

        let stmt = Statement::from_sql_and_values(db.get_database_backend(), &sql, params);
        let rows = db.query_all(stmt).await?;
        let tokens = rows
            .into_iter()
            .map(|row| {
                Ok((
                    row.try_get::<String>("", "token_address")?,
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
