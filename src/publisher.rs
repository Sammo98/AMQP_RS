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
        let properties = Properties::builder()
            // .content_type("Test".into())
            // .content_encoding("Some shitty encoding".into())
            .headers(vec![("abc".into(), Field::LS(LongString("abc".into())))]) // this needs to be a long string for some reason?
            // .delivery_mode(1)
            // .priority(10)
            // .correlation_id("wtf is this working".into())
            // .reply_to("test".into())
            // .expiration("1010".into()) // Looks like this needs to be an int??
            // .message_id("hello".into())
            // .timestamp(20230101)
            // .message_type("some message type".into())
            // .user_id("guest".into()) // this needs to match authenticated user
            // .app_id("test".into())
            // .cluster_id("ha".into())
            .build();
        let content_header = content::Content::new(message.len() as u64, properties);
        let bytes = encode_frame(&content_header).unwrap();
        println!("Content header bytes: {bytes:?}");
        full_buffer.extend_from_slice(&bytes);

        // body
        let mut message_bytes = Vec::new();
        message_bytes.extend_from_slice(message.as_bytes());
        let body = body::Body::new(RawBytes(message_bytes));
        let bytes = encode_frame(&body).unwrap();
        full_buffer.extend_from_slice(&bytes);
        println!("fb {full_buffer:?}");

        self.connection.send(&full_buffer).await?;
        Ok(())
    }
}
