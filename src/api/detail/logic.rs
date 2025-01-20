use crate::api::detail::schema;
use crate::core::AppState;
use crate::entity::{evt_trade_log, kline_5m, token_info, token_summary, token_comment};
use crate::utility::{LibError, LibResult};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder, QuerySelect, ActiveModelTrait};
use chrono::Utc;
use sea_orm::NotSet;
use sea_orm::Set;

pub async fn get_basic_info(
    app_state: AppState,
    token_address: &str,
) -> LibResult<schema::BasicInfoResp> {
    // 获取代币基本信息
    let token = token_info::Entity::find()
        .filter(token_info::Column::TokenAddress.eq(token_address))
        .one(&app_state.db_pool)
        .await?
        .ok_or_else(|| LibError::ParamError("Token not found".to_string()))?;

    // 获取代币市场信息
    let summary = token_summary::Entity::find_by_id(token_address)
        .one(&app_state.db_pool)
        .await?;

    Ok(schema::BasicInfoResp {
        name: token.name,
        symbol: token.symbol,
        icon: token.icon,
        price: summary.as_ref().and_then(|s| s.price),
        price_rate24h: summary.as_ref().and_then(|s| s.price_rate24h),
        market_cap: summary.as_ref().and_then(|s| s.market_cap),
        liquidity: summary.as_ref().and_then(|s| s.liquidity),
        volume24h: summary.as_ref().and_then(|s| s.volume_rate24h),
        total_supply: token.total_supply,
        description: token.description,
        tag: token.tag,
        website: token.website,
        twitter: token.twitter,
        telegram: token.telegram,
    })
}

pub async fn get_kline(
    app_state: &AppState,
    token_address: &str,
    last_open_ts: Option<i64>,
    limit: Option<u64>,
) -> LibResult<schema::KlineResp> {
    let mut query =
        kline_5m::Entity::find().filter(kline_5m::Column::TokenAddress.eq(token_address));

    // 添加时间戳过滤条件
    if let Some(ts) = last_open_ts {
        query = query.filter(kline_5m::Column::OpenTs.lt(ts));
    }

    // 按时间倒序并限制返回数量
    let klines = query
        .order_by_desc(kline_5m::Column::OpenTs)
        .limit(limit.unwrap_or(100))
        .all(&app_state.db_pool)
        .await?;

    let list = klines
        .into_iter()
        .map(|k| schema::KlineData {
            open_ts: k.open_ts,
            close_ts: k.close_ts,
            open: k.open,
            high: k.high,
            low: k.low,
            close: k.close,
            volume: k.volume,
            amount: k.amount,
            txn_num: k.txn_num,
        })
        .collect();

    Ok(schema::KlineResp { list })
}

pub async fn comment_history(
    app_state: AppState,
    token_address: &str,
) -> LibResult<schema::CommentHistoryResp> {
    let comments = token_comment::Entity::find()
        .filter(token_comment::Column::TokenAddress.eq(token_address))
        .order_by_desc(token_comment::Column::CreateTs)
        .limit(20)
        .all(&app_state.db_pool)
        .await?;

    let list = comments
        .into_iter()
        .map(|comment| schema::CommentHistoryData {
            id: comment.id,
            user_address: comment.user_address,
            comment: comment.comment,
            create_ts: comment.create_ts,
        })
        .collect();

    Ok(schema::CommentHistoryResp { list })
}

pub async fn comment_submit(
    app_state: AppState,
    user_address: String,
    token_address: String,
    comment: String,
) -> LibResult<String> {
    // 创建评论
    let comment = token_comment::ActiveModel {
        id: NotSet,
        token_address: Set(token_address),
        user_address: Set(user_address),
        comment: Set(comment),
        create_ts: Set(Utc::now().timestamp()),
    };

    // 保存到数据库
    match comment.insert(&app_state.db_pool).await {
        Ok(_) => Ok("Comment successful".to_string()),
        Err(_) => Err(LibError::CommentFailed),
    }
}

pub async fn get_trade_log(
    app_state: &AppState,
    token_address: &str,
    last_block_number: Option<i64>,
    last_txn_index: Option<i64>,
    last_log_index: Option<i64>,
    limit: Option<u64>,
) -> LibResult<schema::TradeLogResp> {
    let mut query =
        evt_trade_log::Entity::find().filter(evt_trade_log::Column::TokenAddress.eq(token_address));

    // 使用复合主键作为游标
    if let (Some(block_num), Some(txn_idx), Some(log_idx)) =
        (last_block_number, last_txn_index, last_log_index)
    {
        query = query.filter(
            Condition::any()
                .add(evt_trade_log::Column::BlockNumber.lt(block_num))
                .add(
                    Condition::all()
                        .add(evt_trade_log::Column::BlockNumber.eq(block_num))
                        .add(evt_trade_log::Column::TxnIndex.lt(txn_idx)),
                )
                .add(
                    Condition::all()
                        .add(evt_trade_log::Column::BlockNumber.eq(block_num))
                        .add(evt_trade_log::Column::TxnIndex.eq(txn_idx))
                        .add(evt_trade_log::Column::LogIndex.lt(log_idx)),
                ),
        );
    }

    // 按主键排序
    let trades = query
        .order_by_desc(evt_trade_log::Column::BlockNumber)
        .order_by_desc(evt_trade_log::Column::TxnIndex)
        .order_by_desc(evt_trade_log::Column::LogIndex)
        .limit(limit.unwrap_or(20))
        .all(&app_state.db_pool)
        .await?;

    let list = trades
        .into_iter()
        .map(|trade| schema::TradeLogData {
            block_number: trade.block_number,
            txn_index: trade.txn_index,
            log_index: trade.log_index,
            user_address: trade.user_address,
            trace_type: trade.trace_type,
            token0: trade.token0,
            amount0: trade.amount0,
            token1: trade.token1,
            amount1: trade.amount1,
            block_time: trade.block_time,
            txn_hash: trade.txn_hash,
        })
        .collect();

    Ok(schema::TradeLogResp { list })
}
