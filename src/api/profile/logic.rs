use crate::api::profile::schema;
use crate::core::{consts, AppState};
use crate::entity::{token_info, token_summary, User, UserSummary};
use crate::utility::{LibError, LibResult, with_domain};
use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter};

pub async fn get_user_info(
    app_state: AppState,
    address: String,
) -> LibResult<schema::UserInfoResp> {
    let user = User::find_by_id(&address)
        .one(&app_state.db_pool)
        .await?
        .ok_or(LibError::UserNotFound)?;

    Ok(schema::UserInfoResp {
        address: user.address,
        name: user.name,
        avatar: with_domain(&user.avatar),
        create_ts: user.create_ts,
    })
}

pub async fn get_token_owned(
    app_state: AppState,
    address: String,
    query: schema::TokenOwnedQuery,
) -> LibResult<schema::TokenOwnedResp> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);
    let tokens =
        UserSummary::find_token_owned(&app_state.db_pool, address, query.keyword, page, page_size)
            .await?;

    let list = tokens
        .into_iter()
        .map(|(icon, symbol, quantity, value)| schema::TokenOwned {
            token_icon: with_domain(&icon),
            token_symbol: symbol,
            quantity,
            value,
        })
        .collect();

    Ok(schema::TokenOwnedResp { list })
}

pub async fn get_token_created(
    app_state: AppState,
    address: String,
    query: schema::TokenCreatedQuery,
) -> LibResult<schema::TokenCreatedResp> {
    let mut condition = Condition::all();
    condition = condition.add(token_info::Column::UserAddress.eq(address));
    // 基础过滤条件
    if let Some(keyword) = query.keyword {
        condition = condition.add(
            Condition::any()
                .add(token_info::Column::TokenAddress.contains(&keyword))
                .add(token_info::Column::Name.contains(&keyword))
                .add(token_info::Column::Symbol.contains(&keyword)),
        );
    }

    // 构建查询
    let query_builder = token_info::Entity::find()
        .find_also_related(token_summary::Entity)
        .filter(condition);

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
            icon: with_domain(&token.icon),
            tag: token.tag,
            user_address: token.user_address,
            name: token.name,
            symbol: token.symbol,
            description: token.description,
            market_cap: summary.as_ref().map(|s| s.market_cap).unwrap_or_default(),
            is_launched: token.is_launched,
        })
        .collect();

    Ok(schema::TokenCreatedResp { list, total })
}
