use byteflow::{Consumer, Message, Properties, Publisher};

fn handler(message: Message) {
    println!("Printing from handler: {:?}", message.bytes);
    println!("Props: {:?}", message.properties);
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut publisher = Publisher::new("127.0.0.1:5672").await;

    publisher.create_channel().await?;

    publisher.create_queue("my_queue").await?;
    publisher
        .send_message(
            "test from rust",
            "my_queue",
            "",
            false,
            false,
            Properties::default(),
        )
        .await?;
    // let mut consumer = Consumer::new("127.0.0.1:5672").await;

    // consumer.create_channel().await?;
    // consumer.create_queue("test1").await?;
    // consumer.consume_on_queue("test1", &handler).await?;

    Ok(())
}
