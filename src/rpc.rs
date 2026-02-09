use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;

const DEFAULT_RPC_URL: &str = "https://api-tn10.kaspa.org";

#[derive(Error, Debug)]
pub enum RpcError {
    #[error("Connection error: {0}")]
    Connection(String),
    #[error("RPC error: {0}")]
    Rpc(String),
    #[error("JSON error: {0}")]
    JsonError(String),
}

pub struct RpcClient {
    url: String,
    client: reqwest::Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBalanceByAddressResponse {
    pub balance: u64,
}

impl RpcClient {
    pub fn new(rpc_url: Option<&str>) -> Self {
        let url = rpc_url.unwrap_or(DEFAULT_RPC_URL).trim_end_matches('/').to_string();
        Self {
            url,
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Failed to build HTTP client"),
        }
    }

    pub async fn get_balance_by_address(&self, address: &str) -> Result<GetBalanceByAddressResponse, RpcError> {
        let url = format!("{}/addresses/{}/balance", self.url, address);

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| RpcError::Connection(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(RpcError::Rpc(format!("HTTP {}: {}", status, text)));
        }

        let balance_response: RestBalanceResponse = response
            .json()
            .await
            .map_err(|e| RpcError::JsonError(e.to_string()))?;

        Ok(GetBalanceByAddressResponse {
            balance: balance_response.balance,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RestBalanceResponse {
    balance: u64,
}
