use byteflow::{Consumer, Publisher};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handler = |x: &[u8]| {
        println!("Printing from handler: {x:?}");
    };
    // let publisher = Publisher::new("127.0.0.1:5672").await; // Need to delay the connection to avoid tcp timeout

    // publisher.connect().await?;
    // publisher.create_channel().await?;
    // publisher.create_queue("my_queue").await?;
    // publisher
    //     .send_message("test from rust", "my_queue", "", false, false)
    //     .await?;
    let consumer = Consumer::new("127.0.0.1:5672").await; // Need to delay the connection to avoid tcp timeout

    consumer.connect().await?;
    consumer.create_channel().await?;
    // consumer.create_queue("test1").await?;
    consumer
        .consume_on_queue("test1", Arc::new(handler))
        .await?;

    Ok(())
}
