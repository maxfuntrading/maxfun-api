use crate::core::AppState;
use axum::{
    routing::{get, post},
    Router,
};

mod auth;
mod home;
mod profile;

pub fn router() -> Router<AppState> {
    Router::new()
        // Auth routes group
        .nest(
            "/auth",
            Router::new()
                .route("/nonce", get(auth::view::nonce))
                .route("/verify", post(auth::view::verify))
                .route("/logout", post(auth::view::logout)),
        )
        // Home routes group
        .nest(
            "/home",
            Router::new()
                .route("/marquee", get(home::view::get_marquee))
                .route("/token-list", get(home::view::get_token_list))
                .route("/token-tag", get(home::view::get_token_tags)),
        )
        .nest(
            "/profile",
            Router::new()
                .route("/userinfo", get(profile::view::userinfo))
                .route("/token-owned", get(profile::view::get_token_owned))
                .route("/token-created", get(profile::view::get_token_created)),
        )
}
