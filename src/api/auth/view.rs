use axum::extract::{Json, State};
use axum::response::IntoResponse;

use super::logic;
use super::schema;
use crate::core::state::AppState;
use crate::utility::{LibResult, Resp200};

pub async fn nonce() -> LibResult<impl IntoResponse> {
    let rsp = logic::nonce().await?;
    tracing::info!("wallet_login, rsp {:?}", rsp);
    Ok(Resp200::new(rsp))
}

pub async fn verify(
    State(app_state): State<AppState>,
    Json(payload): Json<schema::VerifyReq>,
) -> LibResult<impl IntoResponse> {
    tracing::info!("verify, payload {:?}", payload);
    let rsp = logic::verify(app_state, payload).await?;
    tracing::info!("verify, rsp {:?}", rsp);
    Ok(Resp200::new(rsp))
}

pub async fn logout() -> LibResult<impl IntoResponse> {
    tracing::info!("logout");
    Ok(Resp200::new("logout"))
}
