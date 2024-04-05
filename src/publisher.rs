use crate::common::ClientConnection;
use crate::encde::*;
use crate::frame::*;
use std::cell::RefCell;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub struct Publisher {
    connection: ClientConnection,
    channel: RefCell<u16>,
}

impl Publisher {
    pub async fn new(address: &str) -> Self {
        let connection = ClientConnection::new(address).await;

        Self {
            connection,
            channel: RefCell::new(0),
        }
    }

    fn get_channel(&self) -> u16 {
        self.channel.clone().into_inner()
    }

    pub async fn connect(&self) -> Result<()> {
        self.connection.connect().await?;
        self.channel.replace(1);
        Ok(())
    }

    pub async fn create_channel(&self) -> Result<()> {
        self.connection.create_channel().await?;
        Ok(())
    }

    pub async fn create_queue(&self, queue_name: &str) -> Result<()> {
        self.connection.create_queue(queue_name).await?;
        Ok(())
    }

    pub async fn send_message(
        &self,
        message: &str,
        queue: &str,
        exchange: &str,
        mandatory: bool,
        immediate: bool,
    ) -> Result<()> {
        let mut full_buffer: Vec<u8> = Vec::new();
        let publish = basic::Publish::new(
            ShortString(exchange.into()),
            ShortString(queue.into()),
            Bits(vec![mandatory as u8, immediate as u8]),
        );
        let bytes = encode_frame(&publish).unwrap();
        full_buffer.extend_from_slice(&bytes);

        // Content header
        let properties = Properties::builder().content_type("Hello".into()).build();
        let content_header = content::Content::new(message.len() as u64, properties);
        let bytes = encode_frame(&content_header).unwrap();
        full_buffer.extend_from_slice(&bytes);

        // body
        let mut message_bytes = Vec::new();
        message_bytes.extend_from_slice(message.as_bytes());
        let body = body::Body::new(RawBytes(message_bytes));
        let bytes = encode_frame(&body).unwrap();
        full_buffer.extend_from_slice(&bytes);

        self.connection.send(&full_buffer).await?;
        Ok(())
    }
}
