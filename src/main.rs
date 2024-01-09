mod client;
mod requester;
mod types;
use crate::client::jsonrpc_client::JSONRPCClient;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::runtime::Runtime;
use std::io;

fn fetch_headers(id: String, url_new: String) -> Result<(), Box<dyn std::error::Error>> {

    let cli = JSONRPCClient::new(&url_new, 1337, id).map_err(|e| e as Box<dyn std::error::Error>)?;
    let start = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64 * 1000;
    let end = start - 120 * 1000;

    let res = match cli.get_block_headers_by_start(start, end) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    println!("{:?}", res.get_blocks()[0]);

    Ok(())
}

fn main() {
    println!("Hello World");
    let mut id = String::new();
    let mut url_new = String::new();
    println!("Please enter your chain ID:");
    io::stdin().read_line(&mut id).expect("Failed to read line");
    println!("Please enter your URL:");
    io::stdin().read_line(&mut url_new).expect("Failed to read line");
    match fetch_headers(id.trim().to_string(), url_new.trim().to_string()) {
        Ok(_) => println!("fetch_headers succeeded"),
        Err(err) => println!("fetch_headers error occurred: Check for a valid chain id and url {:?}", err),
    }
}