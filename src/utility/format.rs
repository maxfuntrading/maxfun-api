use chrono::{DateTime, Utc, FixedOffset};
use std::time::Duration;
use ethers::providers::{Http, Middleware, Provider};
use crate::utility::{LibError, LibResult};

pub fn format_name(addr: &str) -> String {
    format!("{}...{}", &addr[..5], &addr[addr.len() - 3..])
}

pub fn get_current_utc8_time() -> String {
    let utc_time = Utc::now();
    let offset = FixedOffset::east_opt(8 * 3600);
    let time_in_utc8: DateTime<FixedOffset> = utc_time.with_timezone(&offset.unwrap());
    let formated_time = time_in_utc8.format("%Y-%m-%d %H:%M").to_string();
    formated_time
}


pub fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}", minutes, seconds)       // mm:ss
}

pub async fn get_chain_id(rpc_url: &str) -> LibResult<i64> {
    let provider = Provider::<Http>::try_from(rpc_url)
        .map_err(|_| LibError::ParamError("Invalid RPC URL".to_string()))?;
    
    let chain_id = provider.get_chainid().await
        .map_err(|e| LibError::EthersError(e))?
        .as_u64() as i64;

    Ok(chain_id)
}