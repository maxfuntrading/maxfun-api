use chrono::{DateTime, Utc, FixedOffset};
use std::time::Duration;

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