use crate::core::{auth, consts};
use axum::{
    extract::DefaultBodyLimit,
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
    middleware, Router,
};
use tower_http::cors::CorsLayer;
mod api;
mod core;
mod entity;
mod utility;

#[tokio::main]
async fn main() {
    utility::log::init();
    let app_state = core::state::init_state().await;

    let mut cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::OPTIONS,
            Method::PUT,
        ])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    if let Ok(origins_str) = dotenvy::var("ORIGINS") {
        if !origins_str.is_empty() {
            let origins: Vec<_> = origins_str.split(',').map(|h| h.parse().unwrap()).collect();
            cors = cors.allow_origin(origins);
        }
    } else {
        tracing::warn!("ORIGINS not found in .env, allow all origins");
        cors = cors.allow_origin(tower_http::cors::Any);
    }

    let app = Router::new()
        .nest("/api", api::router())
        .layer(middleware::from_fn(auth::auth))
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024))
        .layer(cors)
        .with_state(app_state);

    // Read environment variables
    let addr = format!(
        "{}:{}",
        consts::RUN_HOST.as_str(),
        consts::RUN_PORT.as_str()
    );
    // Start server
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
