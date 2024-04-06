use byteflow::{Consumer, Publisher};

fn handler(message: &[u8]) {
    println!("Printing from handler: {message:?}");
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut publisher = Publisher::new("127.0.0.1:5672").await; // Need to delay the connection to avoid tcp timeout

    publisher.connect().await?;
    publisher.create_channel().await?;
    publisher.create_queue("new_queue_pls").await?;
    publisher
        .send_message("test from rust", "new_queue_pls", "", false, false)
        .await?;
    let mut consumer = Consumer::new("127.0.0.1:5672").await; // Need to delay the connection to avoid tcp timeout

    consumer.connect().await?;
    consumer.create_channel().await?;
    consumer.create_queue("test1").await?;
    consumer.consume_on_queue("test1", &handler).await?;

    Ok(())
}
