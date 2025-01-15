use crate::core::AppState;
use axum::{
    routing::{get, post},
    Router,
};

mod auth;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/login", get(auth::view::login))
        .route("/auth/verify", post(auth::view::verify))
        .route("/auth/logout", post(auth::view::logout))
        .route("/auth/userinfo", get(auth::view::userinfo))
}
