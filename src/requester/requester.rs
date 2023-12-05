use std::collections::HashMap;
use std::error::Error; 
use reqwest::{Client, Url, RequestBuilder, header};
use serde::{Serialize, Deserialize};
use serde_json::json;
use tokio::time::{timeout, Duration};
use http::HeaderMap;
use async_trait::async_trait;
use serde_json::from_value;
use serde_json::Value;
use serde::de::DeserializeOwned;

#[derive(Clone, Debug)]
pub struct Options {
    headers: HeaderMap,
    query_params:HashMap<String, String>,
}

impl Options {
    pub fn new() -> Self {
        Options {
            headers: header::HeaderMap::new(),
            query_params: HashMap::new(),
        }
    }
    pub fn is_some(&self) -> bool {
        true
    }
    pub fn unwrap(self) -> Self {
        self
    }
    pub fn with_header(mut self, key: &str, val: &str) -> Self {
        self.headers.insert(
            key.parse::<http::header::HeaderName>().unwrap(), 
            http::header::HeaderValue::from_str(val).unwrap()
        );
        self
    }
    pub fn with_query_params(mut self, key: &str, val: &str) -> Self {
        self.query_params.insert(key.to_string(), val.to_string());
        self
    }
}

#[async_trait]
pub trait JsonReq {
    async fn send_json<T: Serialize + Send + Sync>(self, value: T) -> reqwest::Result<reqwest::Response>;
}


#[async_trait]
impl JsonReq for RequestBuilder {
    async fn send_json<T: Serialize + Send + Sync>(self, value: T) -> reqwest::Result<reqwest::Response> {
        self.header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&value).unwrap())
            .send().await
    }
}

#[derive(Debug)]
pub struct EndpointRequester {
    client: Client,
    uri: Url,
    base: String,
}

impl EndpointRequester {
    pub fn new(uri: Url, base: String) -> Self {
        EndpointRequester {
            client: Client::new(),
            uri,
            base,
        }
    }

    pub async fn send_request<T: Serialize + ?Sized, R: DeserializeOwned>(
        &self,
        method: &str,
        params: &T,
        reply: &mut R,
        options: Options,
    ) -> Result<(), Box<dyn Error>> {
        let mut uri = self.uri.clone();
        if options.is_some() {
            let options_clone = options.clone();
            let options = options_clone.unwrap();
            for (key, val) in options.query_params {
                let key_slice = &key[..];
                let val_slice = &val;
                uri.query_pairs_mut().append_pair(key_slice, val_slice);
            }
        }
        let request_body = json!({
            "jsonrpc": "2.0",
            "method": format!("{}.{}", self.base, method),
            "params": params,
            "id": "0",
        });
    
        let timeout_duration = Duration::from_secs(10);
    
        let response = timeout(timeout_duration, async {
            self.client
                .post(uri.clone())
                .headers(options.headers.clone())
                .send_json(request_body.clone())
                .await
        })
        .await??;
    
        let status = response.status();
    
        if !status.is_success() {
            let all = response.text().await?;
            return Err(format!("received status code: {} {} {}", status, all, uri).into());
        }
    
        let response_body: Value = response.json().await?;
        let result_value = response_body["result"].clone();
        *reply = from_value(result_value)?;
    
        Ok(())
    }
    
}