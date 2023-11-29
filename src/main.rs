mod client;
mod requester;
mod types;

use std::time::{SystemTime, UNIX_EPOCH};
use tokio::runtime::Runtime;
use crate::client::jsonrpc_client::JSONRPCClient;


async fn fetch_headers() -> Result<(), Box<dyn std::error::Error>> {
    let id = "2Wh2uXEnuSVWYJLho5PnGEXeFbWEh7sS1C8JRs12eZfJBy5MAf";
    let url_new = "http://127.0.0.1:64896/ext/bc/2Wh2uXEnuSVWYJLho5PnGEXeFbWEh7sS1C8JRs12eZfJBy5MAf";
    let cli = JSONRPCClient::new(url_new, 1337, id.to_string());
    println!("CLI: {:?}", cli);
    let start = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64 * 1000;
    println!("Start: {:?}", start);
    let end = start - 120 * 1000;
    println!("End: {:?}", end);
    let res = cli?.get_block_headers_by_start(start, end).await?;
    println!("Res: {:?}", res);
    
    println!("{:?}", res.get_blocks()[0]);

    Ok(())
}

fn main() {
    println!("Hello World");
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        if let Err(err) = fetch_headers().await {
            println!("Error occurred: {:?}", err);
        }
    });
}