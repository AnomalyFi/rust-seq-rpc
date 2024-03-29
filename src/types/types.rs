
use std::default::Default;
use serde:: { Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockInfo {
    #[serde(rename = "id")]
    pub block_id: String, 
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
    #[serde(rename = "l1_head")]
    pub l1_head: u64,
    #[serde(rename = "height")]
    pub height: u64,
}

impl Default for BlockInfo {
    fn default() -> Self {
        Self {
            block_id: String::new(),
            timestamp: 0,
            l1_head: 0,
            height: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockHeadersResponse {
    #[serde(rename = "from")]
    pub from: u64,
    #[serde(rename = "blocks")]
    pub blocks: Vec<BlockInfo>,
    #[serde(rename = "prev")]
    pub prev: BlockInfo,
    #[serde(rename = "next")]
    pub next: BlockInfo,
}

impl BlockHeadersResponse {
    pub fn get_blocks(&self) -> &Vec<BlockInfo> {
        &self.blocks
    }
}

impl Default for BlockHeadersResponse {
    fn default() -> Self {
        Self {
            from: 0,
            blocks: Vec::new(),
            prev: BlockInfo::default(),
            next: BlockInfo::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SEQTransaction {
    #[serde(rename = "namespace")]
    pub namespace: String,
    #[serde(rename = "tx_id")]
    pub tx_id: String,
    #[serde(rename = "tx_index")]
    pub index: u64,
    #[serde(rename = "transaction", with = "serde_bytes_ng")]
    pub transaction: Vec<u8>
}

impl Default for SEQTransaction {
    fn default() -> Self {
        Self {
            namespace: String::new(),
            tx_id: String::new(),
            index: 0,
            transaction: Vec::new(),
        }
    }
}

//TODO need to fix this. Tech debt
#[derive(Serialize, Deserialize, Debug)]
pub struct SEQTransactionResponse {
    #[serde(rename = "txs")]
    pub txs: Vec<SEQTransaction>,
    #[serde(rename = "id")]
    pub block_id: String,
}

impl Default for SEQTransactionResponse {
    fn default() -> Self {
        Self {
            txs: Vec::new(),
            block_id: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBlockHeadersByHeightArgs {
    #[serde(rename = "height")]
    pub height: u64, 
    #[serde(rename = "end")]
    pub end: i64,
}

impl Default for GetBlockHeadersByHeightArgs {
    fn default() -> Self {
        Self {
             height: 0,
             end: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBlockHeadersIDArgs {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "end")]
    pub end: i64,
}

impl Default for GetBlockHeadersIDArgs {
    fn default() -> Self {
        Self {
             id: String::new(),
             end: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBlockHeadersByStartArgs {
    #[serde(rename = "start")]
    pub start: i64,
    #[serde(rename = "end")]
    pub end: i64,
}

impl Default for GetBlockHeadersByStartArgs {
    fn default() -> Self {
        Self {
             start: 0,
             end: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBlockTransactionsArgs {
    #[serde(rename = "block_id")]
    pub id: String,
}

impl Default for GetBlockTransactionsArgs {
    fn default() -> Self {
        Self {
            id: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBlockTransactionsByNamespaceArgs {
    #[serde(rename = "height")]
    pub height: u64,
    #[serde(rename = "namespace")]
    pub namespace: String,
}

impl Default for GetBlockTransactionsByNamespaceArgs {
    fn default() -> Self {
        Self {
            height: 0,
            namespace: String::new(),
        }
    }
}