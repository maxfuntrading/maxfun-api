use super::pool;
use super::consts;


#[derive(Clone, Debug)]
pub struct AppState {
    pub db_pool: pool::DB,
    pub redis_pool: pool::RedisPool,
}

pub async fn init_state() -> AppState {
    let db_pool = pool::create_db_pool(consts::PG_URL.as_str()).await;
    let redis_pool = pool::create_redis_pool(consts::REDIS_URL.as_str()).await;

    AppState {
        db_pool,
        redis_pool,
    }
}


#[derive(Clone)]
pub struct ReqContext {
    pub user_addr: String,
}

