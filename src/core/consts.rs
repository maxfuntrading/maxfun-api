use once_cell::sync::Lazy;

pub static RUN_HOST: Lazy<String> =
    Lazy::new(|| std::env::var("RUN_HOST").expect("env not found RUN_HOST"));
pub static RUN_PORT: Lazy<String> =
    Lazy::new(|| std::env::var("RUN_PORT").expect("env not found RUN_PORT"));
pub static JWT_SECRET: Lazy<String> =
    Lazy::new(|| std::env::var("JWT_SECRET").expect("env not found JWT_SECRET"));
pub static JWT_KID: Lazy<String> =
    Lazy::new(|| std::env::var("JWT_KID").expect("env not found JWT_KID"));
pub static PG_URL: Lazy<String> =
    Lazy::new(|| std::env::var("PG_URL").expect("env not found PG_URL"));
pub static REDIS_URL: Lazy<String> =
    Lazy::new(|| std::env::var("REDIS_URL").expect("env not found REDIS_URL"));

// AWS related configuration
pub static AWS_REGION: Lazy<String> =
    Lazy::new(|| std::env::var("AWS_REGION").unwrap_or_else(|_| "ap-southeast-1".to_string()));
pub static AWS_S3_BUCKET: Lazy<String> =
    Lazy::new(|| std::env::var("AWS_S3_BUCKET").expect("env not found AWS_S3_BUCKET"));
pub static AWS_ACCESS_KEY_ID: Lazy<String> =
    Lazy::new(|| std::env::var("AWS_ACCESS_KEY_ID").expect("env not found AWS_ACCESS_KEY_ID"));
pub static AWS_SECRET_ACCESS_KEY: Lazy<String> = Lazy::new(|| {
    std::env::var("AWS_SECRET_ACCESS_KEY").expect("env not found AWS_SECRET_ACCESS_KEY")
});
pub static AWS_S3_ENDPOINT: Lazy<String> =
    Lazy::new(|| std::env::var("AWS_S3_ENDPOINT").expect("env not found AWS_S3_ENDPOINT"));

// Chain related configuration
pub static RPC_URL: Lazy<String> =
    Lazy::new(|| std::env::var("RPC_URL").expect("env not found RPC_URL"));
pub static EOA_PRIVATE_KEY: Lazy<String> =
    Lazy::new(|| std::env::var("EOA_PRIVATE_KEY").expect("env not found EOA_PRIVATE_KEY"));

// Minimum raised amount (USD)
pub static MIN_RAISED_AMOUNT_USD: Lazy<i64> = Lazy::new(|| {
    std::env::var("MIN_RAISED_AMOUNT_USD")
        .unwrap_or_else(|_| "2000".to_string())
        .parse()
        .unwrap_or(2000)
});

// Minimum token total supply
pub static MIN_TOKEN_TOTAL_SUPPLY: Lazy<i64> = Lazy::new(|| {
    std::env::var("MIN_TOKEN_TOTAL_SUPPLY")
        .unwrap_or_else(|_| "1000000".to_string())
        .parse()
        .unwrap_or(1000000)
});

// Default token total supply
pub static DEFAULT_TOKEN_TOTAL_SUPPLY: Lazy<i64> = Lazy::new(|| {
    std::env::var("DEFAULT_TOKEN_TOTAL_SUPPLY")
        .unwrap_or_else(|_| "1000000".to_string())
        .parse()
        .unwrap_or(1000000)
});

// JWT valid duration
pub const JWT_LIVE: i64 = 60 * 60 * 24 * 7;
// JWT near expiration duration
pub const JWT_EXPT: i64 = 60 * 60 * 24;

// Authentication routes
pub const NO_AUTH_ROUTERS: [&str; 16] = [
    "/api/auth/nonce",
    "/api/auth/verify",
    "/api/home/marquee",
    "/api/home/token-list",
    "/api/token/detail/basic-info",
    "/api/token/detail/kline",
    "/api/token/detail/comment-history",
    "/api/token/detail/trade-log",
    "/api/token/detail/holder-distribution",
    "/api/ranking/process",
    "/api/ranking/gainer",
    "/api/ranking/market-cap",
    "/api/ranking/trading-volume",
    "/api/common/tag",
    "/api/common/raised-token",
    "/api/launcher/raised-token-price",
];

// File upload configuration
pub const MAX_UPLOAD_SIZE: usize = 4 * 1024 * 1024; // 4MB
pub const ALLOWED_IMAGE_TYPES: [&str; 4] = ["image/jpeg", "image/png", "image/gif", "image/webp"];
