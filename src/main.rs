mod client;
mod requester;
mod types;

use std::time::{SystemTime, UNIX_EPOCH};
use tokio::runtime::Runtime;
use context::Context;
use crate::client::jsonrpc_client::JSONRPCClient;
// use context::Context;


async fn fetch_headers() -> Result<(), Box<dyn std::error::Error>> {
    let id = "2qMoscnJNq7h9XkLzWBGdFmvSMhnctXfHbiifQSfNN7shyA8SR";
    let url_new = "http://127.0.0.1:41887/ext/bc/2qMoscnJNq7h9XkLzWBGdFmvSMhnctXfHbiifQSfNN7shyA8SR";
    let ctx = Context::new(); 
    let cli = JSONRPCClient::new(url_new, 1337, id.to_string());
    let start = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64 * 1000;
    let end = start - 120 * 1000;
    let res = cli?.get_block_headers_by_start(ctx, start, end).await?;

    println!("{:?}", res.blocks[0]);

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