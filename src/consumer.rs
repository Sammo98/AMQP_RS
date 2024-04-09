use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;

use crate::*;
use crate::{client_connection::Connection, encde::*, frame::*};

pub struct Consumer {
    connection: client_connection::Connection,
}

impl Consumer {
    pub async fn new(address: &str) -> Self {
        let connection = Connection::connect(address).await;

        Self { connection }
    }

    pub async fn create_channel(&mut self) -> Result<()> {
        self.connection.create_channel().await?;
        Ok(())
    }

    pub async fn create_queue(&mut self, queue_name: &str) -> Result<()> {
        self.connection.create_queue(queue_name).await?;
        Ok(())
    }
    pub async fn consume_on_queue(&mut self, queue: &str, handler: Handler) -> Result<()> {
        let consume = basic::Consume::new(queue);
        let bytes = encode_frame(&consume).unwrap();
        self.connection.write(bytes).await;

        // ConsumeOk
        let buffer = self.connection.read().await.unwrap();
        let _consume_ok: basic::ConsumeOk = decode_frame(&buffer).unwrap();
        let (tx, rx) = mpsc::unbounded_channel();
        let sender = self.connection.get_writer();
        tokio::task::spawn(async move { consumer_task(rx, sender, handler).await });

        while let Some(buffer) = self.connection.read().await {
            let header: Header = decode_frame(&buffer).unwrap();
            if header.frame_type == FrameType::Heartbeat {
                let heart_beat = [8_u8, 0, 0, 0, 0, 0, 0, 0xCE].to_vec();
                self.connection.write(heart_beat).await;
                println!("Sent heartbeat");
                continue;
            } else {
                let mut frames = buffer.split_inclusive(|&x| x == FRAME_END);
                let deliver: basic::Deliver = decode_frame(frames.next().unwrap()).unwrap();
                let delivery_tag = deliver.delivery_tag;
                let content_header: content::Content =
                    decode_frame(frames.next().unwrap()).unwrap();
                let properties = content_header.properties;
                let body: body::Body = decode_frame(frames.next().unwrap()).unwrap();
                let bytes = body.content.0;
                let message = Message::new(bytes, properties, AdditionalInfo::new(delivery_tag));
                let _x = tx.send(message);
            }
        }
        Ok(())
    }
}

async fn consumer_task(
    mut receiver: UnboundedReceiver<Message>,
    sender: UnboundedSender<Bytes>,
    handler: Handler,
) {
    println!("Consumer started");
    while let Some(message) = receiver.recv().await {
        let s = sender.clone();
        tokio::task::spawn(async move {
            // Auto ack mode before
            let ack = basic::Ack::new(message.additional_info.delivery_tag);
            let bytes = encode_frame(ack).unwrap();
            s.send(bytes).unwrap();
            println!("Sent ack");
            handler(message);
        });
    }
}
