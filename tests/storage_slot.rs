use base64::decode;
use nodekit_seq_sdk::client::jsonrpc_client::JSONRPCClient;
#[test]
fn test_storage_slot() {
    let uri = "http://127.0.0.1:9656/ext/bc/24rD8QqELL2RF9LkeZ2cZ4qyUbdBsj7VPQyXQnXjQVSccJCusC";
    let network_id: u32 = 1337;
    let chain_id: String = "24rD8QqELL2RF9LkeZ2cZ4qyUbdBsj7VPQyXQnXjQVSccJCusC".to_string();
    let client = JSONRPCClient::new(uri, network_id, chain_id).unwrap();
    let address_str = "v2RWCQjGNxebmArs1TsBdCANHzWDMiEjDMavzKtEXabfGN7um".to_string();
    let slot: u64 = 9;
    let resp = client.get_storage_slot_data(address_str, slot).unwrap();
    let decoded_bytes = decode(&resp.data).expect("Failed to decode base64");
    let res_data = vec![13 as u8, 14, 16, 0];
    assert_eq!(decoded_bytes, res_data);
}
