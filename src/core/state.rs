use super::pool;
use super::consts;


#[derive(Clone, Debug)]
pub struct AppState {
    pub db_pool: pool::DB,
    pub redis_pool: pool::RedisPool,
    pub s3_client: pool::S3Client,
}

pub async fn init_state() -> AppState {
    let db_pool = pool::create_db_pool(consts::PG_URL.as_str()).await;
    let redis_pool = pool::create_redis_pool(consts::REDIS_URL.as_str()).await;
    let s3_client = pool::create_s3_client(
        consts::AWS_ACCESS_KEY_ID.as_str(),
        consts::AWS_SECRET_ACCESS_KEY.as_str(),
        consts::AWS_REGION.as_str(),
        consts::AWS_S3_ENDPOINT.as_str(),
    ).await;

    AppState {
        db_pool,
        redis_pool,
        s3_client,
    }
}


#[derive(Clone)]
pub struct ReqContext {
    pub user_addr: String,
}

