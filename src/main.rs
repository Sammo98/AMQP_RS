#![allow(clippy::unused_io_amount)] // TODO

use std::sync::Arc;
mod client;
mod constants;
mod endec;
mod frame;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = crate::client::Client::new("127.0.0.1:5672").await;
    client.connect().await?;
    client.create_queue("test_queue").await?;
    client
        .send_message("Hello World!", "test_queue", "", false, false)
        .await?;
    // let handler = |x: String| {
    //     println!("Printing from handler: {x}");
    // };
    // client
    //     .consume_on_queue("test_queue", Arc::new(handler))
    //     .await?;
    Ok(())
}
