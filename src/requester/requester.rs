// use bytes::Bytes;
use std::collections::HashMap;
use std::error::Error;
use reqwest::{Client, Url, header};
use serde::{Serialize, Deserialize};
use serde_json::json;
use tokio::time::{timeout, Duration};
use http::HeaderMap;
// use std::str::FromStr;
use serde_json::from_value;
use serde_json::Value;

struct Options {
    headers: HeaderMap,
    query_params:HashMap<String, String>,
}

impl<F> Options {
    pub fn new() -> Self {
        Options {
            headers: header::HeaderMap::new(),
            query_params: HashMap::new(),
        }
    }

    pub fn with_header(mut self, key: &str, val: &str) -> Self {
        self.headers.insert(key.parse::<F>().unwrap(), val.parse().unwrap());
        self
    }
    pub fn with_query_params(mut self, key: &str, val: &str) -> Self {
        self.query_params.insert(key.to_string(), val.to_string());
        self
    }
}

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

    pub async fn send_request<T: Serialize + ?Sized, R: Deserialize<'_>>(
        &self,
        method: &str,
        params: &T,
        options: Options,
    ) -> Result<R, Box<dyn Error>> {
        let mut uri = self.uri.clone();
        for (key, val) in options.query_params {
            uri.query_pairs_mut().append_pair(&key, &val);
        }
        let request_body = json! ({
            "jsonrpc": "2.0",
            "method": format!("{}.{}", self.base, method),
            "params": params, 
            "id": "0",
        });
        let response = timeout(
            Duration::from_secs(10),
            self.client.post(uri)
                .headers(options.headers)
                .json(&request_body)
                .send()
            ).await??;

            let status = response.status();

            if !status.is_success() {
                return Err(("recieved status code: {}", status).into());
            }

            let response_body: Value = response.json().await?;
            let result: R = from_value(response_body["result"].clone())?;

            Ok (result)
    }
}