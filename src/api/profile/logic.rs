use crate::api::profile::schema;
use crate::core::AppState;
use crate::entity::{token_info, token_summary, User, UserSummary};
use crate::utility::{with_domain, LibError, LibResult};
use sea_orm::prelude::Expr;
use sea_orm::sea_query::extension::postgres::PgExpr;
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
    let (tokens, total) =
        UserSummary::find_token_owned(&app_state.db_pool, address, query.keyword, page, page_size)
            .await?;

    let list = tokens
        .into_iter()
        .map(
            |(token_address, icon, symbol, quantity, value)| schema::TokenOwned {
                token_address,
                token_icon: with_domain(&icon),
                token_symbol: symbol,
                quantity,
                value,
            },
        )
        .collect();

    Ok(schema::TokenOwnedResp { list, total })
}

pub async fn get_token_created(
    app_state: AppState,
    address: String,
    query: schema::TokenCreatedQuery,
) -> LibResult<schema::TokenCreatedResp> {
    let mut condition = Condition::all()
        .add(Expr::col((token_info::Entity, token_info::Column::TokenAddress)).ne(""));

    condition = condition.add(token_info::Column::UserAddress.eq(address));
    // Basic filter conditions
    if let Some(keyword) = query.keyword {
        // Check if it's a valid Ethereum address format (42 chars hex starting with 0x)
        let is_eth_address = keyword.len() == 42
            && keyword.starts_with("0x")
            && keyword[2..].chars().all(|c| c.is_ascii_hexdigit());

        if is_eth_address {
            // Exact match for token_address, case insensitive
            condition = condition.add(
                Expr::col((token_info::Entity, token_info::Column::TokenAddress)).ilike(&keyword),
            );
        } else {
            // Fuzzy match for other fields, case insensitive
            condition = condition.add(
                Condition::any()
                    .add(
                        Expr::col((token_info::Entity, token_info::Column::Name))
                            .ilike(&format!("%{}%", keyword)),
                    )
                    .add(
                        Expr::col((token_info::Entity, token_info::Column::Symbol))
                            .ilike(&format!("%{}%", keyword)),
                    ),
            );
        }
    }

    // Build query
    let query_builder = token_info::Entity::find()
        .find_also_related(token_summary::Entity)
        .filter(condition);

    // Handle pagination
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);
    let paginator = query_builder.paginate(&app_state.db_pool, page_size);

    // Get total count
    let total = paginator.num_items().await?;

    // Get current page data
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
            bonding_curve: summary
                .as_ref()
                .map(|s| s.bonding_curve)
                .unwrap_or_default(),
            is_launched: token.is_launched,
        })
        .collect();

    Ok(schema::TokenCreatedResp { list, total })
}
