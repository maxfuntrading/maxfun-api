use super::schema::{self, SortField, SortOrder};
use crate::core::AppState;
use crate::entity::{token_info, token_summary, EvtTradeLog};
use crate::utility::LibResult;
use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

pub async fn get_marquee(app_state: AppState) -> LibResult<schema::MarqueeListResp> {
    let trades = EvtTradeLog::find_latest_trades(&app_state.db_pool).await?;

    let list = trades
        .into_iter()
        .map(
            |(user_address, trade_type, token_address, amount, icon, symbol, tag)| {
                schema::MarqueeItem {
                    user_address,
                    trade_type,
                    token_address,
                    amount,
                    icon,
                    symbol,
                    tag,
                }
            },
        )
        .collect();

    Ok(schema::MarqueeListResp { list })
}

pub async fn get_token_list(
    app_state: AppState,
    query: schema::TokenListQuery,
) -> LibResult<schema::TokenListResp> {
    let mut condition = Condition::all();

    // 基础过滤条件
    if let Some(keyword) = query.keyword {
        condition = condition.add(
            Condition::any()
                .add(token_info::Column::TokenAddress.contains(&keyword))
                .add(token_info::Column::Name.contains(&keyword))
                .add(token_info::Column::Symbol.contains(&keyword)),
        );
    }

    if let Some(tag) = query.tag {
        condition = condition.add(token_info::Column::Tag.eq(tag));
    }

    if let Some(is_launched) = query.is_launched {
        condition = condition.add(token_info::Column::IsLaunched.eq(is_launched));
    }

    // 构建查询
    let mut query_builder = token_info::Entity::find()
        .find_also_related(token_summary::Entity)
        .filter(condition);

    // 排序处理
    match query.sort_by {
        Some(SortField::LaunchTs) => {
            query_builder = match query.sort_order.unwrap_or(SortOrder::Desc) {
                SortOrder::Asc => query_builder.order_by_asc(token_info::Column::LaunchTs),
                SortOrder::Desc => query_builder.order_by_desc(token_info::Column::LaunchTs),
            };
        }
        Some(SortField::Volume24h) => {
            query_builder = match query.sort_order.unwrap_or(SortOrder::Desc) {
                SortOrder::Asc => query_builder.order_by_asc(token_summary::Column::Volume24h),
                SortOrder::Desc => query_builder.order_by_desc(token_summary::Column::Volume24h),
            };
        }
        Some(SortField::MarketCap) => {
            query_builder = match query.sort_order.unwrap_or(SortOrder::Desc) {
                SortOrder::Asc => query_builder.order_by_asc(token_summary::Column::MarketCap),
                SortOrder::Desc => query_builder.order_by_desc(token_summary::Column::MarketCap),
            };
        }
        Some(SortField::LastTrade) => {
            query_builder = match query.sort_order.unwrap_or(SortOrder::Desc) {
                SortOrder::Asc => query_builder.order_by_asc(token_summary::Column::LastTradeTs),
                SortOrder::Desc => query_builder.order_by_desc(token_summary::Column::LastTradeTs),
            };
        }
        None => {
            query_builder = query_builder.order_by_desc(token_info::Column::CreateTs);
        }
    }

    // 分页处理
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);
    let paginator = query_builder.paginate(&app_state.db_pool, page_size);

    // 获取总数
    let total = paginator.num_items().await?;

    // 获取当前页数据
    let tokens = paginator.fetch_page(page - 1).await?;

    let list = tokens
        .into_iter()
        .map(|(token, summary)| schema::TokenInfo {
            token_address: token.token_address,
            icon: token.icon,
            tag: token.tag,
            user_address: token.user_address,
            name: token.name,
            symbol: token.symbol,
            description: token.description,
            market_cap: summary.as_ref().map(|s| s.market_cap).unwrap_or_default(),
            bonding_curve: summary
                .as_ref()
                .map(|s| s.bonding_curve)
                .unwrap_or_default(),
            is_launched: token.is_launched,
        })
        .collect();

    Ok(schema::TokenListResp { list, total })
}
