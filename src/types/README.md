# Introduction

This is a JSON RPC Client for SEQ without any of the dependencies of AvalancheGo, HyperSDK, or SEQ to ensure compatibility with rollups. The types are defined below.

## Structs

| Struct Name | Fields |
|-------------|--------|
| BlockInfo | block_id: String, timestamp: i64, l1_head: u64, height: u64 |
| BlockHeadersResponse | from: u64, blocks: Vec<BlockInfo>, prev: BlockInfo, next: BlockInfo |
| SEQTransaction | namespace: String, tx_id: String, index: u64, transaction: Vec<u8> |
| SEQTransactionResponse | txs: Vec<SEQTransaction>, block_id: String |
| GetBlockHeadersByHeightArgs | height: u64, end: i64 |
| GetBlockHeadersIDArgs | id: String, end: i64 |
| GetBlockHeadersByStartArgs | start: i64, end: i64 |
| GetBlockTransactionsArgs | id: String |
| GetBlockTransactionsByNamespaceArgs | height: u64, namespace: String |

## Methods

| Method Name | Stuct | Parameters | Return Type |
|-------------|-----------|------------|-------------|
| default     | structName | None       | structType   |
| get_blocks  | BlockHeadersResponse | None | &Vec<BlockInfo> |

## Description

### BlockInfo

`BlockInfo` is a struct that represents the information of a block. It has the following fields:

- `block_id`: The id of the block.
- `timestamp`: The timestamp of the block.
- `l1_head`: The L1 head of the block.
- `height`: The height of the block.

### BlockHeadersResponse

`BlockHeadersResponse` is a struct that represents the response of block headers. It has the following fields:

- `from`: The starting point of the block header.
- `blocks`: A vector of `BlockInfo`.
- `prev`: The previous `BlockInfo`.
- `next`: The next `BlockInfo`.

### SEQTransaction

`SEQTransaction` is a struct that represents a SEQ transaction. It has the following fields:

- `namespace`: The namespace of the transaction.
- `tx_id`: The id of the transaction.
- `index`: The index of the transaction.
- `transaction`: The transaction data.

### SEQTransactionResponse

`SEQTransactionResponse` is a struct that represents a SEQ transaction response. It has the following fields:

- `txs`: A vector of `SEQTransaction`.
- `block_id`: The id of the block.

### GetBlockHeadersByHeightArgs

`GetBlockHeadersByHeightArgs` is a struct that represents the arguments for getting block headers by height. It has the following fields:

- `height`: The height of the block.
- `end`: The end of the block.

### GetBlockHeadersIDArgs

`GetBlockHeadersIDArgs` is a struct that represents the arguments for getting block headers by id. It has the following fields:

- `id`: The id of the block.
- `end`: The end of the block.

### GetBlockHeadersByStartArgs

`GetBlockHeadersByStartArgs` is a struct that represents the arguments for getting block headers by start. It has the following fields:

- `start`: The start of the block.
- `end`: The end of the block.

### GetBlockTransactionsArgs

`GetBlockTransactionsArgs` is a struct that represents the arguments for getting block transactions. It has the following field:

- `id`: The ID of the block.

### GetBlockTransactionsByNamespaceArgs

`GetBlockTransactionsByNamespaceArgs` is a struct that represents the arguments for getting block transactions by namespace. It has the following fields:

- `height`: The height of the block.
- `namespace`: The namespace of the block.
