use crate::core::consts;

pub mod jwt;
mod error;
pub mod resp;
pub mod log;
pub mod format;
pub mod http;

pub use error::LibError;
pub use resp::Resp200;

pub type LibResult<T> = Result<T, LibError>;

pub fn with_domain(path: &str) -> String {
    format!("{}{}", consts::AWS_S3_ENDPOINT.as_str(), path)
}
