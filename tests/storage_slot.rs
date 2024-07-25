use std::vec;

use nodekit_seq_sdk::client::jsonrpc_client::JSONRPCClient;
#[test]
fn test_storage_slot() {
    let uri = "http://127.0.0.1:9656/ext/bc/uPkz1qGwZ8aZLUougkmPMfnmoNZV6XXSKBMSE3jRAHJ6RFZUT";
    let network_id: u32 = 1337;
    let chain_id: String = "uPkz1qGwZ8aZLUougkmPMfnmoNZV6XXSKBMSE3jRAHJ6RFZUT".to_string();
    let client = JSONRPCClient::new(uri, network_id, chain_id).unwrap();
    let address_str = "QHYeeRqQdiCxWPs3xvvVzpYWQofFxo8j6KT7qVa2b2iDoLqbw".to_string();
    let slot = 3u32.to_be_bytes().to_vec();
    let resp = client.get_storage_slot_data(address_str, slot).unwrap();
    // let decoded_bytes = decode(&resp.data).expect("Failed to decode base64");
    println!("{:?}", resp);
    let res_data = vec![1 as u8, 2, 3, 56, 32];
    assert_eq!(resp.data, res_data);
}
