use axum::extract::{Json, State};
use axum::http::{header::SET_COOKIE, HeaderMap};
use axum::response::IntoResponse;
use axum_extra::{headers, TypedHeader};

use super::logic;
use super::schema;
use crate::core::state::AppState;
use crate::utility::{LibError, LibResult, Resp200};

static COOKIE_NAME: &str = "nonce";

pub async fn nonce() -> LibResult<impl IntoResponse> {
    let rsp = logic::nonce().await?;
    tracing::info!("wallet_login, rsp {:?}", rsp);

    let nonce = &rsp.nonce;
    // when develop on local, we could use Lax
    // let cookie = format!("{COOKIE_NAME}={nonce}; SameSite=Lax; Path=/");
    let cookie = format!("{COOKIE_NAME}={nonce}; SameSite=None; Secure; Path=/");
    // Set cookie
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        cookie.parse().map_err(|_| LibError::CookieInvalid)?,
    );

    Ok((headers, Resp200::new(rsp)))
}

pub async fn verify(
    State(app_state): State<AppState>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
    Json(payload): Json<schema::VerifyReq>,
) -> LibResult<impl IntoResponse> {
    tracing::info!("verify, payload {:?}", payload);
    let cookie = cookies
        .get(COOKIE_NAME)
        .ok_or_else(|| LibError::CookieInvalid)?;
    let nonce = cookie.to_string();
    let rsp = logic::verify(app_state, payload, nonce).await?;
    tracing::info!("verify, rsp {:?}", rsp);
    Ok(Resp200::new(rsp))
}

pub async fn logout() -> LibResult<impl IntoResponse> {
    tracing::info!("logout");
    Ok(Resp200::new("logout"))
}
