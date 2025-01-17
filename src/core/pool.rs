use aws_config::BehaviorVersion;
use aws_credential_types::Credentials;
use aws_sdk_s3::{Client as SdkS3Client, config::Region};
use redis::Client;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;
use tracing::log;

pub type DB = DatabaseConnection;
pub type RedisPool = Client;
pub type S3Client = SdkS3Client;

pub async fn create_db_pool(pg_url: &str) -> DB {
    let mut opt = ConnectOptions::new(pg_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    Database::connect(opt).await.expect("could not create db_pool due to")
}

pub async fn create_redis_pool(redis_url: &str) -> RedisPool {
    Client::open(redis_url).expect("could not create redis client")
}

pub async fn create_s3_client(access_key: &str, secret_key: &str, region: &str, endpoint: &str) -> S3Client {
    let credentials = Credentials::new(access_key, secret_key, None, None, "static");
    
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(region.to_string()))
        .credentials_provider(credentials)
        .load()
        .await;
    
    SdkS3Client::new(&config)
}
