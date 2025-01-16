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
pub static S3_AVATAR_URI: Lazy<String> =
    Lazy::new(|| std::env::var("S3_AVATAR_URI").expect("env not found S3_AVATAR_URI"));

// jwt 有效时长
pub const JWT_LIVE: i64 = 60 * 60 * 24 * 7;
// jwt 快过期时长
pub const JWT_EXPT: i64 = 60 * 60 * 24;

// 鉴权路由
pub const NO_AUTH_ROUTERS: [&str; 2] = ["/api/auth/nonce", "/api/auth/verify"];
pub const SVC_AUTH_TOKEN: &str = "f2jv330PCK564jKsIZ6I7Y8jiOW83Jw5SsbmJZe9LOz2bglVz0eHA99LtG22c1U6";