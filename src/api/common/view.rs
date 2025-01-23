use crate::api::common::logic;
use crate::core::AppState;
use crate::utility::{LibError, LibResult, Resp200};
use axum::extract::State;
use axum::response::IntoResponse;
use axum_extra::extract::Multipart;

pub async fn get_tags(State(app_state): State<AppState>) -> LibResult<impl IntoResponse> {
    let rsp = logic::get_tags(app_state).await?;
    Ok(Resp200::new(rsp))
}

pub async fn get_raised_token(State(app_state): State<AppState>) -> LibResult<impl IntoResponse> {
    let rsp = logic::get_raised_tokens(app_state).await?;
    Ok(Resp200::new(rsp))
}

pub async fn upload_icon(
    State(app_state): State<AppState>,
    mut multipart: Multipart,
) -> LibResult<impl IntoResponse> {
    // 获取上传的文件
    let field = multipart
        .next_field()
        .await
        .map_err(|e| {
            tracing::error!("Failed to get multipart field: {:?}", e);
            LibError::UploadFailed
        })?
        .ok_or(LibError::NoFileUploaded)?;

    // 获取文件信息
    let file_name = field
        .file_name()
        .ok_or(LibError::ParamError("Invalid param".to_string()))?
        .to_string();

    let content_type = field
        .content_type()
        .ok_or(LibError::ParamError("Invalid param".to_string()))?
        .to_string();

    let bytes = field
        .bytes()
        .await
        .map_err(|e| {
            tracing::error!("Failed to read file bytes: {:?}", e);
            LibError::UploadFailed
        })?
        .to_vec();

    // 上传文件
    let rsp = logic::upload_icon(app_state, file_name, content_type, bytes).await?;
    Ok(Resp200::new(rsp))
}
