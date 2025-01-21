use crate::core::AppState;
use axum::{
    routing::{get, post},
    Router,
};

mod auth;
mod common;
mod detail;
mod home;
mod launcher;
mod profile;
mod ranking;

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
                .route("/token-list", get(home::view::get_token_list)),
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
                .route(
                    "/raised-token-price",
                    get(launcher::view::get_raised_token_price),
                ),
        )
        .nest(
            "/token/detail",
            Router::new()
                .route("/basic-info", get(detail::view::basic_info))
                .route("/kline", get(detail::view::kline))
                .route("/comment-history", get(detail::view::comment_history))
                .route("/comment-submit", post(detail::view::comment_submit))
                .route("/trade-log", get(detail::view::trade_log))
                .route(
                    "/holder-distribution",
                    get(detail::view::holder_distribution),
                ),
        )
        .nest(
            "/ranking",
            Router::new()
                .route("/process", get(ranking::view::process_ranking))
                .route("/gainer", get(ranking::view::gainer_ranking))
                .route("/market-cap", get(ranking::view::market_cap_ranking))
                .route("/trading-volume", get(ranking::view::trading_volume_ranking)),
        )
}
