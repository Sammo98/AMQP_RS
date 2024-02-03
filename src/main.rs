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
    let handler = |x: String| println!("Printing from handler: {x}");
    client.consume_on_queue("test_queue", handler).await?;
    Ok(())
}
