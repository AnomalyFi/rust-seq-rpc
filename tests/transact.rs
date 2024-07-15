use nodekit_seq_sdk::client::jsonrpc_client::JSONRPCClient;
#[test]
fn test_submit_transact_tx() {
    let uri = "http://127.0.0.1:9654/ext/bc/24rD8QqELL2RF9LkeZ2cZ4qyUbdBsj7VPQyXQnXjQVSccJCusC";
    let network_id: u32 = 1337;
    let chain_id: String = "24rD8QqELL2RF9LkeZ2cZ4qyUbdBsj7VPQyXQnXjQVSccJCusC".to_string();
    let client = JSONRPCClient::new(uri, network_id, chain_id).unwrap();
    let function_name = String::from("transact");
    let contract_address = String::from("v2RWCQjGNxebmArs1TsBdCANHzWDMiEjDMavzKtEXabfGN7um");
    let input = vec![13 as u8, 14, 16, 0];
    let resp = client
        .submit_transact_tx(function_name, contract_address, input)
        .unwrap();
    println!("tx id: {:?}", resp.tx_id);
}
