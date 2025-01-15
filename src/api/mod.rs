use crate::core::AppState;
use axum::{
    routing::{get, post},
    Router,
};

mod auth;
mod home;

pub fn router() -> Router<AppState> {
    Router::new()
        // Auth routes group
        .nest(
            "/auth",
            Router::new()
                .route("/login", get(auth::view::login))
                .route("/verify", post(auth::view::verify))
                .route("/logout", post(auth::view::logout))
                .route("/userinfo", get(auth::view::userinfo)),
        )
        // Home routes group
        .nest(
            "/home",
            Router::new()
                .route("/marquee", get(home::view::get_marquee))
                .route("/token-list", get(home::view::get_token_list))
                .route("/token-tag", get(home::view::get_token_tags)),
        )
}
