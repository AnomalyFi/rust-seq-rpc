mod client;
mod requester;
mod types;

use crate::client::jsonrpc_client::JSONRPCClient;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::runtime::Runtime;
use std::io;

async fn fetch_headers(id: String, url_new: String) -> Result<(), Box<dyn std::error::Error>> {
    //error is in cli when it calls JSONRPClient 
    let cli = JSONRPCClient::new(&url_new, 1337, id);
    // println!("CLI: {:?}", cli);
    let start = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64 * 1000;
    // println!("Start: {:?}", start);
    let end = start - 120 * 1000;
    // println!("End: {:?}", end);
    let res = match cli?.get_block_headers_by_start(start, end).await {
        Ok(res) => res,
        Err(err) => return Err(err.into()),
    };
    // println!("Res: {:?}", res);
    
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

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        if let Err(err) = fetch_headers(id.trim().to_string(), url_new.trim().to_string()).await {
            println!("fetch_headers error occurred: {:?}", err);
        }
    });
}
