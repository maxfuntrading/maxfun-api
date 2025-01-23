use crate::api::ranking::schema;
use crate::core::AppState;
use crate::entity::{token_info, token_summary};
use crate::utility::LibResult;
use sea_orm::{EntityTrait, JoinType, QueryOrder, QuerySelect, RelationTrait};

const TOP_LIMIT: u64 = 10;

pub async fn get_process_ranking(app_state: AppState) -> LibResult<schema::RankingResp> {
    let tokens = token_info::Entity::find()
        .find_also_related(token_summary::Entity)
        .order_by_desc(token_summary::Column::BondingCurve)
        .limit(TOP_LIMIT)
        .all(&app_state.db_pool)
        .await?;

    let list = tokens_to_ranking_items(tokens);
    Ok(schema::RankingResp { list })
}

pub async fn get_gainer_ranking(app_state: AppState) -> LibResult<schema::RankingResp> {
    let tokens = token_info::Entity::find()
        .find_also_related(token_summary::Entity)
        .order_by_desc(token_summary::Column::PriceRate24h)
        .limit(TOP_LIMIT)
        .all(&app_state.db_pool)
        .await?;

    let list = tokens_to_ranking_items(tokens);
    Ok(schema::RankingResp { list })
}

pub async fn get_market_cap_ranking(app_state: AppState) -> LibResult<schema::RankingResp> {
    let tokens = token_info::Entity::find()
        .find_also_related(token_summary::Entity)
        .order_by_desc(token_summary::Column::MarketCap)
        .limit(TOP_LIMIT)
        .all(&app_state.db_pool)
        .await?;

    let list = tokens_to_ranking_items(tokens);
    Ok(schema::RankingResp { list })
}

pub async fn get_volume_ranking(app_state: AppState) -> LibResult<schema::RankingResp> {
    let tokens = token_info::Entity::find()
        .find_also_related(token_summary::Entity)
        .order_by_desc(token_summary::Column::Volume24h)
        .limit(TOP_LIMIT)
        .all(&app_state.db_pool)
        .await?;

    let list = tokens_to_ranking_items(tokens);
    Ok(schema::RankingResp { list })
}

// 辅助函数：将查询结果转换为排名项
fn tokens_to_ranking_items(
    tokens: Vec<(token_info::Model, Option<token_summary::Model>)>,
) -> Vec<schema::RankingItem> {
    tokens
        .into_iter()
        .enumerate()
        .map(|(index, (token, summary))| schema::RankingItem {
            rank: (index + 1) as i32,
            token_address: token.token_address,
            name: token.name,
            icon: token.icon,
            market_cap: summary.as_ref().map(|s| s.market_cap).unwrap_or_default(),
            bonding_curve: summary
                .as_ref()
                .map(|s| s.bonding_curve)
                .unwrap_or_default(),
            price_rate24h: summary
                .as_ref()
                .map(|s| s.price_rate24h)
                .unwrap_or_default(),
            volume_24h: summary.as_ref().map(|s| s.volume_24h).unwrap_or_default(),
        })
        .collect()
}
