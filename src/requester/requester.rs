use std::collections::HashMap;
use std::error::Error; 
use reqwest::{Url, Client};
use reqwest::header::HeaderMap;
use serde::{Serialize};
use serde_json::json;

use serde_json::from_value;
use serde_json::Value;
use serde::de::DeserializeOwned;

#[derive(Clone, Debug)]
pub struct Options {
    pub headers: HeaderMap,
    pub query_params:HashMap<String, String>,
}

impl Options {
    pub fn new() -> Self {
        Options {
            headers: HeaderMap::new(),
            query_params: HashMap::new(),
        }
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

#[derive(Debug)]
pub struct EndpointRequester {
    pub client: reqwest::blocking::Client,
    pub uri: Url,
    pub base: String,
}

impl EndpointRequester {
    pub fn new(uri: Url, base: String) -> Self {
        EndpointRequester {
            client: reqwest::blocking::Client::new(),
            uri,
            base,
        }
    }

    pub fn send_request<T: Serialize + ?Sized, R: DeserializeOwned + std::fmt::Debug>(
        &self,
        method: &str,
        params: &T,
        reply: &mut R,
        options: Options,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut uri = self.uri.clone();
        if !options.headers.is_empty() || !options.query_params.is_empty() {
            for (key, val) in &options.query_params {
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
        
        let response = self.client
                .post(uri.clone())
                .headers(options.headers.clone())
                .json(&request_body)
                .send()?;

        let status = response.status();
    
        if !status.is_success() {
            let all = response.text()?;
            return Err(format!("received status code: {} {} {}", status, all, uri).into());
        }
    
        let response_body: Value = response.json()?;
        let result_value = response_body["result"].clone();

        *reply = match from_value(result_value) {
            Ok(val) => val,
            Err(err) => return Err(Box::new(err)),
        };
        Ok(())
    }
    
}