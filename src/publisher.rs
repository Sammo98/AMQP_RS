use crate::encde::*;
use crate::frame::*;
use crate::tcp::TcpAdapter;
use std::cell::RefCell;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub struct Publisher {
    tcp_adapter: TcpAdapter,
    channel: RefCell<u16>,
}

impl Publisher {
    pub async fn new(address: &str) -> Self {
        let tcp_adapter = TcpAdapter::new(address).await;

        Self {
            tcp_adapter,
            channel: RefCell::new(0),
        }
    }

    fn get_channel(&self) -> u16 {
        self.channel.clone().into_inner()
    }

    pub async fn connect(&mut self) -> Result<()> {
        self.tcp_adapter.connect().await?;
        self.channel.replace(1);
        Ok(())
    }

    pub async fn create_channel(&mut self) -> Result<()> {
        self.tcp_adapter.create_channel().await?; // These methods shouldn't be coupled to the adapter
        Ok(())
    }

    pub async fn create_queue(&mut self, queue_name: &str) -> Result<()> {
        self.tcp_adapter.create_queue(queue_name).await?;
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

        self.tcp_adapter.send(full_buffer).await;
        Ok(())
    }
}
