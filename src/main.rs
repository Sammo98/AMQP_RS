#![allow(clippy::unused_io_amount)] // TODO

use byteflow::client::Client;
use byteflow::Publisher;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut client = Client::new("127.0.0.1:5672").await;
    // client.connect().await?;
    // client.create_queue("my_queue").await?; // This calls open channel, not sure about that

    // client
    //     .send_message("hello!", "my_queue", "", false, false)
    //     .await?;
    // let handler = |x: &[u8]| {
    //     println!("Printing from handler: {x:?}");
    // };
    // client
    //     .consume_on_queue("test_queue", Arc::new(handler))
    //     .await?;

    let publisher = Publisher::new("127.0.0.1:5672").await; // Need to delay the connection to avoid tcp timeout

    publisher.connect().await?;
    publisher.create_channel().await?;
    publisher.create_queue("my_queue").await?;
    publisher
        .send_message("test from rust", "my_queue", "", false, false)
        .await?;

    // client.close().await?;
    Ok(())
}
