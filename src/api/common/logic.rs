use crate::api::common::schema;
use crate::core::{consts, AppState};
use crate::entity::{self, RaisedToken, TagInfo};
use crate::utility::{with_domain, LibError, LibResult};
use aws_sdk_s3::primitives::ByteStream;
use sea_orm::{EntityTrait, QueryOrder};
use uuid::Uuid;

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

pub async fn get_raised_tokens(app_state: AppState) -> LibResult<schema::RaisedTokenListResp> {
    let tokens = RaisedToken::find()
        .order_by_asc(entity::raised_token::Column::CreateTs)
        .all(&app_state.db_pool)
        .await?;

    let list = tokens
        .into_iter()
        .map(|token| schema::RaisedToken {
            address: token.address,
            name: token.name,
            symbol: token.symbol,
            decimal: token.decimal,
            icon: with_domain(&token.icon),
            price: token.price,
            oracle: token.oracle,
        })
        .collect();

    Ok(schema::RaisedTokenListResp { list })
}

pub async fn upload_icon(
    app_state: AppState,
    file_name: String,
    content_type: String,
    bytes: Vec<u8>,
) -> LibResult<schema::UploadIconResp> {
    // Verify file size
    if bytes.len() > consts::MAX_UPLOAD_SIZE {
        return Err(LibError::FileTooLarge);
    }
    // Verify file type
    if !consts::ALLOWED_IMAGE_TYPES.contains(&content_type.as_str()) {
        return Err(LibError::InvalidFileType);
    }
    // Generate unique filename
    let ext = file_name
        .split('.')
        .last()
        .filter(|ext| consts::ALLOWED_IMAGE_TYPES.iter().any(|t| t.ends_with(ext)))
        .unwrap_or("png");
    let key = format!("icon/{}.{}", Uuid::new_v4(), ext);
    // Upload to S3
    app_state
        .s3_client
        .put_object()
        .bucket(consts::AWS_S3_BUCKET.as_str())
        .key(&key)
        .body(ByteStream::from(bytes))
        .content_type(content_type)
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Failed to upload file to S3: {:?}", e);
            LibError::UploadFailed
        })?;

    // Return access URL
    let url = with_domain(&format!("/{}", key));
    Ok(schema::UploadIconResp { url })
}
