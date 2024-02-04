use std::sync::Arc;

mod client;
mod common;
mod communication;
mod constants;
mod errors;
mod method;
mod properties;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = crate::client::Client::new("127.0.0.1:5672").await;
    println!("About to call connect");
    client.connect().await?;
    client.create_queue("test_queue").await?;
    let handler = |x: String| {
        println!("Sleeping!");
        std::thread::sleep(std::time::Duration::from_secs(150));
        println!("Printing from handler: {x}");
    };
    client
        .consume_on_queue("test_queue", Arc::new(handler))
        .await?;
    Ok(())
}
