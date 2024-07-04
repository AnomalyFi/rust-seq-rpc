# Introduction

This is a JSON RPC Client for SEQ without any of the dependencies of AvalancheGo, HyperSDK, or SEQ to ensure compatibility with rollups. The types are defined below.

## Structs

| Struct Name | Fields |
|-------------|--------|
| SubmitMsgTxArgs | chain_id: String, network_id: u32, secondary_chain_id: Vec<u8>, data: Vec<u8> |
| SubmitMsgTxReply | tx_id: String |
| JSONRPCClient | requester: EndpointRequester, network_id: u32, chain_id: String |

## Methods

| Method Name | Structure | Parameters | Return Type |
|-------------|-----------|------------|-------------|
| default     | SubmitMsgTxArgs | None | SubmitMsgTxArgs |
| default     | SubmitMsgTxReply | None | SubmitMsgTxReply |
| new         | JSONRPCClient | uri: &str, network_id: u32, chain_id: String | Result<JSONRPCClient, Box Error> |
| submit_tx   | JSONRPCClient | chain_id: String, network_id: u32, secondary_chain_id: Vec<u8>, data: Vec<u8> | Result<String, Box Error> |
| get_block_headers_by_height | JSONRPCClient | height: u64, end: i64 | Result<BlockHeadersResponse, Box Error> |
| get_block_headers_by_id | JSONRPCClient | id: String, end: i64 | Result<BlockHeadersResponse, Box Error> |
| get_block_headers_by_start | JSONRPCClient | start: i64, end: i64 | Result<BlockHeadersResponse, Box Error> |
| get_block_transactions_by_namespace | JSONRPCClient | height: u64, namespace: String |Result<SEQTransactionResponse, Box Error> |
| get_storage_slot_data | JSONRPCClient | address_str: String, slot: string | Result<StorageSlotResponse, Box Error> |
## Description

### SubmitMsgTxArgs

`SubmitMsgTxArgs` is a struct that represents the arguments for submitting a message transaction. It has the following fields:

- `chain_id`: The id of the chain.
- `network_id`: The id of the network.
- `secondary_chain_id`: The secondary chain id.
- `data`: The transaction data.

### SubmitMsgTxReply

`SubmitMsgTxReply` is a struct that represents the reply for submitting a message transaction. It has the following field:

- `tx_id`: The id of the transaction.

### JSONRPCClient

`JSONRPCClient` is a struct that represents a JSON RPC client. It has the following fields:

- `requester`: The endpoint requester.
- `network_id`: The id of the network.
- `chain_id`: The id of the chain.

Each method returns a `Result` type, which is either the expected return type on success or a Boxed Error on failure.
