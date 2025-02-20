use crate::core::consts;

mod error;
pub mod format;
pub mod http;
pub mod jwt;
pub mod log;
pub mod resp;

pub use error::LibError;
pub use resp::Resp200;

pub type LibResult<T> = Result<T, LibError>;

pub fn with_domain(path: &str) -> String {
    format!("{}{}", consts::AWS_S3_ENDPOINT.as_str(), path)
}
