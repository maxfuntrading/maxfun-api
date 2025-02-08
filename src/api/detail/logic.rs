use crate::api::detail::schema;
use crate::core::AppState;
use crate::entity::{RaisedToken, User};
use crate::entity::{
    evt_trade_log, kline_5m, raised_token, token_comment, token_info, token_summary, user,
    user_summary,
};
use crate::utility::{with_domain, LibError, LibResult};
use chrono::Utc;
use rust_decimal::Decimal;
use sea_orm::NotSet;
use sea_orm::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect,
};

pub async fn get_basic_info(
    app_state: AppState,
    token_address: &str,
) -> LibResult<schema::BasicInfoResp> {
    // 获取代币基本信息
    let token = token_info::Entity::find()
        .filter(token_info::Column::TokenAddress.eq(token_address.to_lowercase()))
        .one(&app_state.db_pool)
        .await?
        .ok_or_else(|| LibError::ParamError("Token not found".to_string()))?;

    // 获取代币市场信息
    let summary = token_summary::Entity::find_by_id(token_address.to_lowercase())
        .one(&app_state.db_pool)
        .await?
        .ok_or_else(|| LibError::ParamError("Token summary not found".to_string()))?;

    // 获取 raised token 信息
    let raised_token = RaisedToken::find_by_id(&token.raised_token)
        .one(&app_state.db_pool)
        .await?
        .ok_or_else(|| LibError::ParamError("Raised token not found".to_string()))?;

    Ok(schema::BasicInfoResp {
        token_basic: schema::TokenBasicInfo {
            name: token.name,
            symbol: token.symbol,
            icon: with_domain(&token.icon),
            description: token.description,
            tag: token.tag,
            website: token.website,
            twitter: token.twitter,
            telegram: token.telegram,
            total_supply: token.total_supply,
            price: summary.price,
            price_rate24h: summary.price_rate24h,
            market_cap: summary.market_cap,
            volume24h: summary.volume_24h,
        },
        raised_token: schema::RaisedTokenInfo {
            address: raised_token.address,
            name: raised_token.name,
            symbol: raised_token.symbol,
            icon: with_domain(&raised_token.icon),
            decimal: raised_token.decimal,
        },
    })
}

pub async fn get_kline(
    app_state: &AppState,
    token_address: &str,
    last_open_ts: Option<i64>,
    limit: Option<u64>,
) -> LibResult<schema::KlineResp> {
    let mut query =
        kline_5m::Entity::find().filter(kline_5m::Column::TokenAddress.eq(token_address.to_lowercase()));

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
    page: Option<u64>,
    page_size: Option<u64>,
) -> LibResult<schema::CommentHistoryResp> {
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(20);

    // 计算总数
    let total = token_comment::Entity::find()
        .filter(token_comment::Column::TokenAddress.eq(token_address.to_lowercase()))
        .count(&app_state.db_pool)
        .await?;

    // 获取分页数据
    let comments = token_comment::Entity::find()
        .find_also_related(user::Entity) // 关联查询用户表
        .filter(token_comment::Column::TokenAddress.eq(token_address.to_lowercase()))
        .order_by_desc(token_comment::Column::CreateTs)
        .offset(((page - 1) * page_size) as u64)
        .limit(page_size)
        .all(&app_state.db_pool)
        .await?;

    let list = comments
        .into_iter()
        .map(|(comment, user)| schema::CommentHistoryData {
            id: comment.id,
            user_address: comment.user_address,
            user_avatar: user.map_or_else(String::new, |u| with_domain(&u.avatar)),
            comment: comment.comment,
            create_ts: comment.create_ts,
        })
        .collect();

    Ok(schema::CommentHistoryResp { list, total })
}

pub async fn comment_submit(
    app_state: AppState,
    user_address: String,
    token_address: String,
    comment: String,
) -> LibResult<schema::CommentHistoryData> {
    // 创建评论
    let comment_model = token_comment::ActiveModel {
        id: NotSet,
        token_address: Set(token_address.clone().to_lowercase()),
        user_address: Set(user_address.clone()),
        comment: Set(comment.clone()),
        create_ts: Set(Utc::now().timestamp()),
    };

    // 保存到数据库
    match comment_model.insert(&app_state.db_pool).await {
        Ok(saved_comment) => {
            // 获取用户信息
            let user = User::find_by_id(&user_address)
                .one(&app_state.db_pool)
                .await?
                .ok_or(LibError::UserNotFound)?;

            Ok(schema::CommentHistoryData {
                id: saved_comment.id,
                user_address,
                user_avatar: with_domain(&user.avatar),
                comment,
                create_ts: saved_comment.create_ts,
            })
        }
        Err(e) => {
            tracing::error!(
                "Failed to insert comment. token: {}, user: {}, error: {:?}",
                token_address,
                user_address,
                e
            );
            Err(LibError::CommentFailed)
        }
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
    // 获取代币信息
    let token = token_info::Entity::find()
        .filter(token_info::Column::TokenAddress.eq(token_address.to_lowercase()))
        .one(&app_state.db_pool)
        .await?
        .ok_or_else(|| LibError::ParamError("Token not found".to_string()))?;

    // 获取募资代币信息
    let raised_token = raised_token::Entity::find_by_id(&token.raised_token)
        .one(&app_state.db_pool)
        .await?
        .ok_or_else(|| LibError::ParamError("Raised token not found".to_string()))?;

    // 查询交易记录
    let mut query =
        evt_trade_log::Entity::find().filter(evt_trade_log::Column::TokenAddress.eq(token_address.to_lowercase()));

    // 使用复合主键作为游标
    match (last_block_number, last_txn_index, last_log_index) {
        (Some(block_num), Some(txn_idx), Some(log_idx)) => {
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
        (None, None, None) => {}
        _ => {
            return Err(LibError::ParamError(
                "Cursor parameters must all be provided or all be omitted".to_string(),
            ));
        }
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
            trade_type: trade.trade_type,
            token1_amount: trade.amount0,
            token2_amount: trade.amount1,
            block_time: trade.block_time,
            txn_hash: trade.txn_hash,
            price: trade.price,
            price_token: trade.price_token,
        })
        .collect();

    Ok(schema::TradeLogResp {
        token1_name: token.name,
        token2_name: raised_token.name,
        list,
    })
}

pub async fn holder_distribution(
    app_state: AppState,
    token_address: &str,
    page: Option<u64>,
    page_size: Option<u64>,
) -> LibResult<schema::HolderDistributionResp> {
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(20);

    // 获取总供应量
    let token = token_summary::Entity::find()
        .filter(token_summary::Column::TokenAddress.eq(token_address.to_lowercase()))
        .one(&app_state.db_pool)
        .await?
        .ok_or_else(|| LibError::ParamError("Token not found".to_string()))?;

    let total_supply = token.total_supply;

    // 获取持有者总数
    let total_holders = user_summary::Entity::find()
        .filter(user_summary::Column::TokenAddress.eq(token_address.to_lowercase()))
        .count(&app_state.db_pool)
        .await?;

    // 获取持有者列表
    let holders = user_summary::Entity::find()
        .filter(user_summary::Column::TokenAddress.eq(token_address.to_lowercase()))
        .order_by_desc(user_summary::Column::Amount)
        .offset(((page - 1) * page_size) as u64)
        .limit(page_size)
        .all(&app_state.db_pool)
        .await?;

    let list = holders
        .into_iter()
        .map(|holder| {
            let percentage = if total_supply > Decimal::ZERO {
                (holder.amount * Decimal::new(100, 0)) / total_supply
            } else {
                Decimal::ZERO
            };

            schema::HolderData {
                user_address: holder.user_address,
                amount: holder.amount,
                percentage,
            }
        })
        .collect();

    Ok(schema::HolderDistributionResp {
        total_holders,
        list,
    })
}
