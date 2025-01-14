use crate::utility::error::LibError;
use anyhow::anyhow;
use reqwest::{header, Client, Error as ReqwestError, Response};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::future::Future;
use tokio::time::{sleep, Duration};
use tracing::{error, warn};

const MAX_RETRIES: u32 = 3;
const BASE_DELAY_MS: u64 = 1000;
const TIMEOUT_SECS: u64 = 90;

#[derive(Debug, Deserialize)]
pub struct SuccessResponse<T> {
    pub code: i32,
    pub msg: String,
    pub data: T,
}

pub struct HttpClient {
    client: Client,
    max_retries: u32,
    base_delay: u64,
}

impl HttpClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(TIMEOUT_SECS))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            max_retries: MAX_RETRIES,
            base_delay: BASE_DELAY_MS,
        }
    }

    // 配置方法
    pub fn with_max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }

    pub fn with_base_delay(mut self, delay: u64) -> Self {
        self.base_delay = delay;
        self
    }

    // 核心请求处理逻辑
    async fn handle_request<T, Fut>(&self, request_fn: impl Fn() -> Fut) -> Result<T, LibError>
    where
        T: DeserializeOwned,
        Fut: Future<Output=Result<Response, ReqwestError>>,
    {
        let mut attempts = 0;

        loop {
            let response = request_fn().await;

            match response {
                Ok(resp) => {
                    let status = resp.status();
                    if status.is_success() {
                        let success_response: SuccessResponse<T> = resp.json().await?;
                        return Ok(success_response.data);
                    }
                    let error_text = resp.text().await.unwrap_or_default();
                    return Err(LibError::Other(anyhow!("HTTP error: {} - {}",status,error_text)));
                }
                Err(err) => {
                    attempts += 1;
                    if attempts >= self.max_retries {
                        return Err(LibError::Other(anyhow!("Request failed after {} attempts: {}",attempts,err)));
                    }
                    warn!("Request error (attempt {}): {}", attempts, err);
                }
            }

            let delay = self.base_delay * 2_u64.pow(attempts);
            warn!("Retrying in {}ms (attempt {}/{})", delay, attempts + 1, self.max_retries);
            sleep(Duration::from_millis(delay)).await;
        }
    }

    pub async fn get<T>(&self, url: &str) -> Result<T, LibError>
    where
        T: DeserializeOwned,
    {
        self.handle_request(|| {
            let client = self.client.clone();
            let url = url.to_string();
            async move { client.get(&url).send().await }
        })
            .await
    }

    pub async fn get_with_auth<T>(&self, url: &str, token: &str) -> Result<T, LibError>
    where
        T: DeserializeOwned,
    {
        self.handle_request(|| {
            let client = self.client.clone();
            let url = url.to_string();
            async move {
                client.get(&url)
                    .header("x-api-key", token.to_string())
                    .send()
                    .await
            }
        })
            .await
    }

    pub async fn post<T, B>(&self, url: &str, body: &B) -> Result<T, LibError>
    where
        T: DeserializeOwned,
        B: serde::Serialize,
    {
        self.handle_request(|| {
            let client = self.client.clone();
            let url = url.to_string();
            let body = serde_json::to_value(body).expect("Failed to serialize request body");
            async move { client.post(&url).json(&body).send().await }
        })
            .await
    }

    pub async fn post_with_auth<T, B>(&self, url: &str, token: &str, body: &B) -> Result<T, LibError>
    where
        T: DeserializeOwned,
        B: serde::Serialize,
    {
        self.handle_request(|| {
            let client = self.client.clone();
            let url = url.to_string();
            let body = serde_json::to_value(body).expect("Failed to serialize request body");
            async move { client.post(&url).header("x-api-key", token.to_string()).json(&body).send().await }
        })
            .await
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}