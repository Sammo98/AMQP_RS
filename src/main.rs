use byteflow::{Consumer, Message, Properties, Publisher};

fn handler(message: Message) {
    println!("Printing from handler: {:?}", message.bytes);
    println!("Props: {:?}", message.properties);
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut publisher = Publisher::new("127.0.0.1:5672").await;

    // publisher.create_channel().await?;

    // publisher.create_queue("new_queue1").await?;
    // publisher
    //     .send_message(
    //         "what up",
    //         "new_queue1",
    //         "",
    //         false,
    //         false,
    //         Properties::default(),
    //     )
    //     .await?;
    let mut consumer = Consumer::new("127.0.0.1:5672").await;

    consumer.create_channel().await?;
    consumer.create_queue("new_queue1").await?;
    consumer.consume_on_queue("new_queue1", &handler).await?;

    Ok(())
}
