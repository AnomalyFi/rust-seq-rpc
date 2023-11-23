use serde:: { Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct BlockInfo {
    #[serde(rename = "id")]
    block_id: String, 
    #[serde(rename = "timestamp")]
    timestamp: i64,
    #[serde(rename = "l1_head")]
    l1_head: u64,
}

#[derive(Serialize, Deserialize)]
pub struct BlockHeadersResponse {
    #[serde(rename = "from")]
    from: u64,
    #[serde(rename = "blocks")]
    blocks: Vec<BlockInfo>,
    #[serde(rename = "prev")]
    prev: BlockInfo,
    #[serde(rename = "next")]
    next: BlockInfo,
}

#[derive(Serialize, Deserialize)]
pub struct SEQTransaction {
    #[serde(rename = "namespace")]
    namespace: String,
    #[serde(rename = "tx_id")]
    tx_id: String,
    #[serde(rename = "tx_index")]
    index: u64,
    #[serde(rename = "transaction")]
    transaction: Vec<u8>
}

//TODO need to fix this. Tech debt
#[derive(Serialize, Deserialize)]
pub struct SEQTransactionResponse {
    #[serde(rename = "txs")]
    txs: Vec<SEQTransaction>,
    #[serde(rename = "id")]
    block_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetBlockHeadersByHeightArgs {
    #[serde(rename = "height")]
    height: u64, 
    #[serde(rename = "end")]
    end: i64,
}

#[derive(Serialize, Deserialize)]
pub struct GetBlockHeadersIDArgs {
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "end")]
    end: i64,
}

#[derive(Serialize, Deserialize)]
pub struct GetBlockHeadersByStartArgs {
    #[serde(rename = "start")]
    start: i64,
    #[serde(rename = "end")]
    end: i64,
}

#[derive(Serialize, Deserialize)]
pub struct GetBlockTransactionsArgs {
    #[serde(rename = "block_id")]
    id: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetBlockTransactionsByNamespaceArgs {
    #[serde(rename = "height")]
    height: u64,
    #[serde(rename = "namespace")]
    namespace: String,
}