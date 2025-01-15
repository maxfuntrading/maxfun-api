use sea_orm::{EntityTrait, QueryOrder, QueryFilter, Condition, ColumnTrait};
use super::schema;
use crate::core::state::AppState;
use crate::entity::{self, TagInfo, EvtTradeLog, TokenInfo};
use crate::utility::*;

pub async fn get_token_tags(app_state: AppState) -> LibResult<schema::TokenTagListResp> {
    let tags = TagInfo::find()
        .order_by_asc(entity::tag_info::Column::Sort)
        .all(&app_state.db_pool)
        .await?;

    let list = tags
        .into_iter()
        .map(|tag| schema::TokenTag {
            name: tag.name,
            sort: tag.sort,
        })
        .collect();

    Ok(schema::TokenTagListResp { list })
}

pub async fn get_marquee(app_state: AppState) -> LibResult<schema::MarqueeListResp> {
    let trades = EvtTradeLog::find_latest_trades(&app_state.db_pool).await?;

    let list = trades
        .into_iter()
        .map(|(user_address, trade_type, token_address, amount, icon, symbol, tag)| {
            schema::MarqueeItem {
                user_address,
                trade_type,
                token_address,
                amount,
                icon,
                symbol,
                tag,
            }
        })
        .collect();

    Ok(schema::MarqueeListResp { list })
}

pub async fn get_token_list(
    app_state: AppState,
    query: schema::TokenListQuery,
) -> LibResult<schema::TokenListResp> {
    let mut condition = Condition::all();

    if let Some(keyword) = query.keyword {
        condition = condition.add(
            Condition::any()
                .add(entity::token_info::Column::TokenAddress.contains(&keyword))
                .add(entity::token_info::Column::Name.contains(&keyword))
                .add(entity::token_info::Column::Symbol.contains(&keyword))
        );
    }

    if let Some(tag) = query.tag {
        condition = condition.add(entity::token_info::Column::Tag.eq(tag));
    }

    if let Some(is_launched) = query.is_launched {
        condition = condition.add(entity::token_info::Column::IsLaunched.eq(is_launched));
    }

    let tokens = TokenInfo::find()
        .filter(condition)
        .order_by_desc(entity::token_info::Column::CreateTs)
        .all(&app_state.db_pool)
        .await?;

    let list = tokens
        .into_iter()
        .map(|token| schema::TokenInfo {
            token_address: token.token_address,
            user_address: token.user_address,
            name: token.name,
            icon: token.icon,
            symbol: token.symbol,
            description: token.description,
            tag: token.tag,
            website: token.website,
            twitter: token.twitter,
            telegram: token.telegram,
            total_supply: token.total_supply,
            raised_token: token.raised_token,
            raised_amount: token.raised_amount,
            sale_ratio: token.sale_ratio,
            reserved_ratio: token.reserved_ratio,
            pool_ratio: token.pool_ratio,
            launch_ts: token.launch_ts,
            maxbuy_amount: token.maxbuy_amount,
            create_ts: token.create_ts,
            is_launched: token.is_launched,
        })
        .collect();

    Ok(schema::TokenListResp { list })
}