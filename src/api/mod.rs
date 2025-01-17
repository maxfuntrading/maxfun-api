use crate::core::AppState;
use axum::{
    routing::{get, post},
    Router,
};

mod auth;
mod home;
mod profile;
mod launcher;
mod common;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest(
            "/auth",
            Router::new()
                .route("/nonce", get(auth::view::nonce))
                .route("/verify", post(auth::view::verify))
                .route("/logout", post(auth::view::logout)),
        )

        .nest(
            "/common",
            Router::new()
                .route("/tag", get(common::view::get_tags))
                .route("/raised-token", get(common::view::get_raised_token))
                .route("/upload-icon", post(common::view::upload_icon)),
        )

        .nest(
            "/home",
            Router::new()
                .route("/marquee", get(home::view::get_marquee))
                .route("/token-list", get(home::view::get_token_list))
        )

        .nest(
            "/profile",
            Router::new()
                .route("/userinfo", get(profile::view::userinfo))
                .route("/token-owned", get(profile::view::get_token_owned))
                .route("/token-created", get(profile::view::get_token_created)),
        )

        .nest(
            "/launcher",
            Router::new()
                .route("/launch-token", post(launcher::view::launch_token))
        )
}
