// mod requester;
// mod types;

use crate::types::{BlockHeadersResponse, GetBlockHeadersByHeightArgs, GetBlockHeadersIDArgs, GetBlockHeadersByStartArgs, SEQTransactionResponse, GetBlockTransactionsByNamespaceArgs};
use crate::requester::requester::EndpointRequester;
use context::Context;
use reqwest::Url;

pub struct JSONRPCClient {
    requester: EndpointRequester,
    network_id: u32,
    chain_id: String,
}

impl JSONRPCClient {
    pub fn new(uri: &str, network_id: u32, chain_id: String) -> Result<Self, Box<dyn
    std::error::Error>> {
        let uri = Url::parse(uri)?;
        let requester = EndpointRequester::new(uri, "tokenvm".to_string());
        Ok(Self {
            requester,
            network_id,
            chain_id,
        })
    }

    pub async fn submit_tx(
        &self,
        ctx: Context,
        chain_id: String,
        network_id: u32,
        secondary_chain_id: Vec<u8>,
        data: Vec<u8>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let args = SubmitMsgTxArgs {
            chain_id,
            network_id,
            secondary_chain_id,
            data,
        };
        let mut resp = SubmitMsgTxReply::default();
        self.requester.send_request(ctx, "submitMsgTx", &args, &mut resp).await?;
        Ok(resp.tx_id)
    }

    pub async fn get_block_headers_by_height(
        &self,
        ctx: Context,
        height: u64,
        end: i64,
    ) -> Result<BlockHeadersResponse, Box<dyn std::error::Error>> {
        let args = GetBlockHeadersByHeightArgs { height, end };
        let mut resp = BlockHeadersResponse::default();
        self.requester.send_request(ctx, "getBlockHeadersByHeight", &args, &mut resp).await?;
        Ok(resp)
    }

    pub async fn get_block_headers_by_id(
        &self,
        ctx: Context,
        id: String,
        end: i64,
    ) -> Result<BlockHeadersResponse, Box<dyn std::error::Error>> {
        let args = GetBlockHeadersIDArgs { id, end };
        let mut resp = BlockHeadersResponse::default();
        self.requester.send_request(ctx, "getBlockHeadersId", &args, &mut resp).await?;
        Ok(resp)
    }

    pub async fn get_block_headers_by_start(
        &self,
        ctx: Context,
        start: i64,
        end: i64,
    ) -> Result<BlockHeadersResponse, Box<dyn std::error::Error>> {
        let args = GetBlockHeadersByStartArgs { start, end };
        let mut resp = BlockHeadersResponse::default();
        self.requester.send_request(ctx, "getBlockHeadersByStart", &args, &mut resp).await?;
        Ok(resp)
    }

    pub async fn get_block_transactions_by_namespace(
        &self,
        ctx: Context,
        height: u64,
        namespace: String,
    ) -> Result<SEQTransactionResponse, Box<dyn std::error::Error>> {
        let args = GetBlockTransactionsByNamespaceArgs { height, namespace };
        let mut resp = SEQTransactionResponse::default();
        self.requester.send_request(ctx, "getBlockTransactions", &args).await?;
        Ok(resp)
    }
}