use crate::types::types::*;
use crate::requester::requester::*;
use reqwest::Url;
use serde:: { Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SubmitMsgTxArgs {
    #[serde(rename = "chain_id")]
    pub chain_id: String,
    #[serde(rename = "network_id")]
    pub network_id: u32,
    #[serde(rename = "secondary_chain_id")]
    pub secondary_chain_id: Vec<u8>,
    #[serde(rename = "data")]
    pub data: Vec<u8>,
}
impl Default for SubmitMsgTxArgs {
    fn default() -> Self {
        Self {
            chain_id: String::new(),
            network_id: 0,
            secondary_chain_id: Vec::new(),
            data: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SubmitMsgTxReply {
    #[serde(rename = "txId")]
    pub tx_id: String,
}

impl Default for SubmitMsgTxReply {
    fn default() -> Self {
        Self {
            tx_id: String::new(),
        }
    }
}

#[derive(Debug)]
pub struct JSONRPCClient {
    pub requester: EndpointRequester,
    pub network_id: u32,
    pub chain_id: String,
}

impl JSONRPCClient {
    pub fn new(uri: &str, network_id: u32, chain_id: String) -> Result<Self, Box<dyn
    std::error::Error + Send + Sync>> {
        let uri = Url::parse(uri)?.to_string();
        let token = uri.clone() + "/tokenapi";
        let parsed_token = Url::parse(&token)?;
        let requester = EndpointRequester::new(parsed_token, "tokenvm".to_string());
        Ok(Self {
            requester,
            network_id,
            chain_id,
        })
    }

    pub fn submit_tx(
        &self,
        chain_id: String,
        network_id: u32,
        secondary_chain_id: Vec<u8>,
        data: Vec<u8>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let args = SubmitMsgTxArgs {
            chain_id,
            network_id,
            secondary_chain_id,
            data,
        };
        let options = Options::new();
        let mut resp: SubmitMsgTxReply = SubmitMsgTxReply::default();
        let _ = self.requester.send_request("submitMsgTx", &args, &mut resp, options);
        Ok(resp.tx_id)
    }

    pub fn get_block_headers_by_height(
        &self,
        height: u64,
        end: i64,
    ) -> Result<BlockHeadersResponse, Box<dyn std::error::Error + Send + Sync>> {
        let args = GetBlockHeadersByHeightArgs { height, end };
        let options = Options::new();
        let mut resp: BlockHeadersResponse = BlockHeadersResponse::default();
        let _ = self.requester.send_request("getBlockHeadersByHeight", &args, &mut resp, options);
        Ok(resp)
    }

    pub fn get_block_headers_by_id(
        &self,
        id: String,
        end: i64,
    ) -> Result<BlockHeadersResponse, Box<dyn std::error::Error + Send + Sync>> {
        let args = GetBlockHeadersIDArgs { id, end };
        let options = Options::new();
        let mut resp:  BlockHeadersResponse = BlockHeadersResponse::default();
        let _ = self.requester.send_request("getBlockHeadersId", &args, &mut resp, options);
        Ok(resp)
    }

    pub fn get_block_headers_by_start(
        &self,
        start: i64,
        end: i64,
    ) -> Result<BlockHeadersResponse, Box<dyn std::error::Error + Send + Sync>> {
        let args = GetBlockHeadersByStartArgs { start, end };
        let options = Options::new();
        let mut resp: BlockHeadersResponse = BlockHeadersResponse::default();
        let _ = self.requester.send_request("getBlockHeadersByStart", &args, &mut resp, options);
        Ok(resp)
    }

    pub fn get_block_transactions_by_namespace(
        &self,
        height: u64,
        namespace: String,
    ) -> Result<SEQTransactionResponse, Box<dyn std::error::Error + Send + Sync>> {
        let args = GetBlockTransactionsByNamespaceArgs { height, namespace };
        let mut resp: SEQTransactionResponse = SEQTransactionResponse::default();
        let options = Options::new();
        let _ = self.requester.send_request("getBlockTransactionsByNamespace", &args, &mut resp, options);
        Ok(resp)
    }
}