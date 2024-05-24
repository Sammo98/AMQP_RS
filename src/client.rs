use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;

use crate::*;
use crate::{client_connection::Connection, encde::*, frame::*};

use self::properties::PropertiesBuilder;

pub struct Client {
    connection: client_connection::Connection,
    channel_id: u16,
}

impl Client {
    pub async fn new(connection_params: ConnectionParameters<'_>) -> Self {
        let connection = Connection::connect(connection_params).await;
        let channel_id = connection.channel_id;

        Self {
            connection,
            channel_id,
        }
    }

    pub async fn create_channel(&mut self) -> Result<()> {
        self.connection.create_channel().await?;
        Ok(())
    }

    pub async fn create_queue(&mut self, queue_definition: QueueDefinition) -> Result<String> {
        let queue = self.connection.create_queue(queue_definition).await?;
        Ok(queue)
    }

    pub async fn create_exchange(
        &mut self,
        exchange: String,
        exchange_type: ExchangeType,
    ) -> Result<()> {
        self.connection
            .create_exchange(&exchange, exchange_type)
            .await?;
        Ok(())
    }

    pub async fn delete_exchange(&mut self, exchange: &str) -> Result<()> {
        self.connection.delete_exchange(exchange).await?;
        Ok(())
    }

    pub async fn send_message(
        &mut self,
        message: &str,
        queue: &str,
        exchange: &str,
        mandatory: bool,
        immediate: bool,
        properties: Properties,
        wait_for_response: bool,
        handler: Handler,
    ) -> Result<()> {
        let mut full_buffer: Vec<u8> = Vec::new();
        let publish = basic::Publish::new(self.channel_id, exchange, queue, mandatory, immediate);
        let bytes = encode_frame(&publish).unwrap();
        full_buffer.extend_from_slice(&bytes);

        let response_queue = match wait_for_response {
            true => {
                let queue_def = QueueDefinition::builder()
                    .queue_name("".into())
                    .exclusive(true)
                    .auto_delete(true)
                    .build();
                let t_queue = &self.create_queue(queue_def).await?;
                let queue = t_queue.to_owned();
                Some(queue)
            }
            false => None,
        };
        println!("Created queue: {response_queue:?}");
        let properties = PropertiesBuilder::builder()
            .reply_to(response_queue.clone().unwrap().to_owned())
            .build();

        // Content header
        let content_header =
            content::Content::new(self.channel_id, message.len() as u64, properties);
        let bytes = encode_frame(&content_header).unwrap();
        full_buffer.extend_from_slice(&bytes);

        // body
        let mut message_bytes = Vec::new();
        message_bytes.extend_from_slice(message.as_bytes());
        let body = body::Body::new(self.channel_id, RawBytes(message_bytes));
        let bytes = encode_frame(&body).unwrap();
        full_buffer.extend_from_slice(&bytes);

        self.connection.write(full_buffer).await;

        if wait_for_response {
            self.consume_on_queue(&response_queue.unwrap(), handler)
                .await?;
        }
        Ok(())
    }

    pub async fn consume_on_queue(&mut self, queue: &str, handler: Handler) -> Result<()> {
        let consume = basic::Consume::new(self.channel_id, queue);
        let bytes = encode_frame(&consume).unwrap();
        self.connection.write(bytes).await;

        // ConsumeOk
        let buffer = self.connection.read().await.unwrap();
        let _consume_ok: basic::ConsumeOk = decode_frame(&buffer).unwrap();
        let (tx, rx) = mpsc::unbounded_channel();
        let sender = self.connection.get_writer();
        let channel_id = self.channel_id.clone();
        tokio::task::spawn(async move { consumer_task(channel_id, rx, sender, handler).await });

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
    channel_id: u16,
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
            let response_queue = message.properties.clone().reply_to;
            let response = handler(message);

            match response_queue {
                Some(queue) => {
                    let message = response.unwrap();

                    let mut full_buffer: Vec<u8> = Vec::new();
                    let publish = basic::Publish::new(channel_id, "", &queue, false, false);
                    let bytes = encode_frame(&publish).unwrap();
                    full_buffer.extend_from_slice(&bytes);

                    let content_header = content::Content::new(
                        channel_id,
                        message.len() as u64,
                        Properties::default(),
                    );
                    let bytes = encode_frame(&content_header).unwrap();
                    full_buffer.extend_from_slice(&bytes);

                    let mut message_bytes = Vec::new();
                    message_bytes.extend_from_slice(&message);
                    let body = body::Body::new(channel_id, RawBytes(message_bytes));
                    let bytes = encode_frame(&body).unwrap();
                    full_buffer.extend_from_slice(&bytes);
                    s.send(full_buffer).unwrap();
                    println!("Sent message!")
                }
                None => {}
            }
        });
    }
}
