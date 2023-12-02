// use bytes::Bytes;
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
    //added to resolve errors as rustc --explain (error number) suggested
    pub fn is_some(&self) -> bool {
        true
    }
    //added to resolve errors as rustc --explain (error number) suggested
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

//'JsonReq' to send the json data. had to create in order for send_json() to execute below in send_request method
#[async_trait]
pub trait JsonReq {
    async fn send_json<T: Serialize + Send + Sync>(self, value: T) -> reqwest::Result<reqwest::Response>;
}


#[async_trait]
impl JsonReq for RequestBuilder {
    async fn send_json<T: Serialize + Send + Sync>(self, value: T) -> reqwest::Result<reqwest::Response> {
        //header sets content type to be application/json so request 
        //is known to be carrying json data.
        self.header(reqwest::header::CONTENT_TYPE, "application/json")
            //converts type T value, which implements Serialize trait, into a JSON string rep.
            .body(serde_json::to_string(&value).unwrap())
            //sends the request we created
            .send().await
    }
}

//holds info for making JSON RPC requests.
#[derive(Debug)]
pub struct EndpointRequester {
    //uses reqwest lib 
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
    //this file could be wrong but i don't think it is since the logic is similar. 
    //If you look at the error message you'll see what each client, uri, and base prints.
    //will take a more in depth look after classes/hw/projects tn 11/30.

    //handles sending JSON RPC requests.
    pub async fn send_request<T: Serialize + ?Sized, R: DeserializeOwned>(
        &self,
        //name of JSON RPC method to be called
        method: &str,
        //represents request parameters
        params: &T,
        //represents response data
        reply: &mut R,
        //contains extra headers and query params
        options: Options,
    ) -> Result<(), Box<dyn Error>> {
        let mut uri = self.uri.clone();
        //makes sure that any query parameters provided in the Options object are properly appended to the URI before sending the request. 
        //this allows for dynamic construction of the request URL with query parameters.
        if options.is_some() {
            //solution from rust debugger. avoid modifying original obbj directly(bad in rust)
            let options_clone = options.clone();
            //unwraps value to access query_params field
            let options = options_clone.unwrap();
            //loop over query_params, extract key/val pair
            for (key, val) in options.query_params {
                let key_slice = &key[..];
                let val_slice = &val;
                //take the pair to uri query parameters
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