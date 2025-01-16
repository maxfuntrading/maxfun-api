use sea_orm::{EntityTrait, QueryOrder};
use crate::api::common::schema;
use crate::core::AppState;
use crate::entity;
use crate::entity::TagInfo;
use crate::utility::LibResult;

pub async fn get_tags(app_state: AppState) -> LibResult<schema::TagListResp> {
    let tags = TagInfo::find()
        .order_by_asc(entity::tag_info::Column::Sort)
        .all(&app_state.db_pool)
        .await?;

    let list = tags
        .into_iter()
        .map(|tag| schema::Tag {
            name: tag.name,
            sort: tag.sort,
        })
        .collect();

    Ok(schema::TagListResp { list })
}