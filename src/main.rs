#![allow(clippy::unused_io_amount)] // TODO

use std::sync::Arc;
mod body;
mod client;
mod common;
mod communication;
mod constants;
mod content;
mod endec;
mod errors;
mod method;
mod properties;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = crate::client::Client::new("127.0.0.1:5672").await;
    client.connect().await?;
    client.create_queue("test_queue").await?;
    // client
    //     .send_message("Hello World!", "test_queue", "", false, false)
    //     .await?;
    let handler = |x: String| {
        println!("Sleeping!");
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("Printing from handler: {x}");
    };
    client
        .consume_on_queue("test_queue", Arc::new(handler))
        .await?;
    Ok(())
}
