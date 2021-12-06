use ethereum_types::H160;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap as Map;
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnsApiError {
    #[error("request error {0}")]
    RequestError(String),
    #[error("response parsing error {0}")]
    ParseError(String),
    #[error("server API error {0}")]
    ApiError(String),
    #[error("unknown client error")]
    Unknown,
}

#[derive(Debug, Clone, Deserialize)]
struct ReverseSingleResponse {
    pub address: H160,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
struct ReverseBulkRequest {
    pub addresses: Vec<H160>,
}

#[derive(Debug, Clone, Deserialize)]
struct ReverseBulkResponse {
    pub result: Map<H160, String>,
}

#[derive(Debug, Clone, Deserialize)]
struct ErrorResponse {
    pub error: String,
}

pub struct Client {
    address: String,
}

impl Client {
    pub fn default() -> Self {
        Self {
            address: "https://enormous.cloud/ens/reverse/".to_owned(),
        }
    }
    pub fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
        }
    }

    pub fn reverse(&self, address: &H160) -> Result<String, EnsApiError> {
        let agent = ureq::AgentBuilder::new()
            .timeout_read(Duration::from_secs(60))
            .timeout_write(Duration::from_secs(5))
            .build();
        let url = format!("{}?address={}", &self.address, address);
        println!("ENS-API-CLIENT URL={}", url);
        let rq = agent.get(&url).set("Content-Type", "application/json");
        let response: String = match rq.call() {
            Ok(x) => x.into_string().unwrap(),
            Err(e) => return Err(EnsApiError::RequestError(e.to_string())),
        };
        println!("ENS-API-CLIENT response={}", response);
        if let Ok(err) = serde_json::from_str::<ErrorResponse>(&response) {
            return Err(EnsApiError::ApiError(err.error));
        };
        let res = match serde_json::from_str::<ReverseSingleResponse>(&response) {
            Ok(x) => x.name,
            Err(e) => return Err(EnsApiError::ParseError(e.to_string())),
        };
        Ok(res)
    }

    pub fn bulk_reverse(&self, addresses: Vec<H160>) -> Result<Map<H160, String>, EnsApiError> {
        let agent = ureq::AgentBuilder::new()
            .timeout_read(Duration::from_secs(60))
            .timeout_write(Duration::from_secs(5))
            .build();

        let payload = serde_json::to_string(&ReverseBulkRequest { addresses }).unwrap();
        println!("ENS-API-CLIENT URL={} PAYLOAD={}", self.address, payload);
        let rq = agent
            .post(&self.address)
            .set("Content-Type", "application/json");
        let response: String = match rq.call() {
            Ok(x) => x.into_string().unwrap(),
            Err(e) => return Err(EnsApiError::RequestError(e.to_string())),
        };
        println!("ENS-API-CLIENT response={}", response);
        if let Ok(err) = serde_json::from_str::<ErrorResponse>(&response) {
            return Err(EnsApiError::ApiError(err.error));
        };
        let res = match serde_json::from_str::<ReverseBulkResponse>(&response) {
            Ok(x) => x.result,
            Err(e) => return Err(EnsApiError::ParseError(e.to_string())),
        };
        Ok(res)
    }
}
