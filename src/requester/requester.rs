// use bytes::Bytes;
use std::collections::HashMap;
use std::error::Error; 
use reqwest::{Client, Url, RequestBuilder, header};
use serde::{Serialize, Deserialize};
use serde_json::json;
use tokio::time::{timeout, Duration};
use http::HeaderMap;
// use std::str::FromStr;
use serde_json::from_value;
use serde_json::Value;
use serde::de::DeserializeOwned;

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
            let options = options.unwrap();
            for (key, val) in options.query_params {
                let key_slice = &key[..];
                let val_slice = &val;
                uri.query_pairs_mut().append_pair(key_slice, val_slice);
                // uri.query_pairs_mut().append_pair(&key, &val);
            }
        }
        let request_body = json!({
            "jsonrpc": "2.0",
            //concates the name of jsonrpc babse+method
            "method": format!("{}.{}", self.base, method),
            "params": params,
            "id": "0",
        });

        let timeout_duration = Duration::from_secs(10);

        //need to look into sending a json request using reqwest library or another library because error 
        //.send_json doesn't exist so i need to make one or find a library that imports sending json requests.
        let response = timeout(timeout_duration, async {
            self.client
                .post(uri)
                .headers(options.headers.clone())
                .send_json(&request_body)
                .await
        })
        .await??;

        let status = response.status();

        if !status.is_success() {
            let all = response.text().await?;
            return Err(format!("received status code: {} {} {}", status, all, uri).into());
        }

        let response_body: Value = response.json().await?;
        *reply = from_value(response_body["result"].clone())?;

        Ok(())
    }
}