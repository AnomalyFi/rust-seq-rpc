use std::vec;

use nodekit_seq_sdk::client::jsonrpc_client::JSONRPCClient;
#[test]
fn test_storage_slot() {
    let uri = "http://127.0.0.1:9656/ext/bc/b32xGvao82zAVUhsjwCjV5d68f9g2Tb3MN2qeJpZezMzNCuUU";
    let network_id: u32 = 1337;
    let chain_id: String = "b32xGvao82zAVUhsjwCjV5d68f9g2Tb3MN2qeJpZezMzNCuUU".to_string();
    let client = JSONRPCClient::new(uri, network_id, chain_id).unwrap();
    let address_str = "v2RWCQjGNxebmArs1TsBdCANHzWDMiEjDMavzKtEXabfGN7um".to_string();
    let slot = vec![1 as u8, 2, 3, 56, 32];
    let resp = client.get_storage_slot_data(address_str, slot).unwrap();
    // let decoded_bytes = decode(&resp.data).expect("Failed to decode base64");
    let res_data = vec![1 as u8, 2, 3, 56, 32];
    assert_eq!(resp.data, res_data);
}
