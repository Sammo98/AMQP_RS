use crate::{client_connection::Connection, encde::*, frame::*, types::*};

pub struct Publisher {
    connection: Connection,
    channel_id: u16,
}

impl Publisher {
    pub async fn new(address: &str) -> Self {
        let connection = Connection::connect(address).await;
        let channel_id = connection.channel_id;
        println!("ch id{channel_id}");

        Self {
            connection,
            channel_id,
        }
    }

    pub async fn create_channel(&mut self) -> Result<()> {
        self.connection.create_channel().await?; // These methods shouldn't be coupled to the adapter
        Ok(())
    }

    pub async fn create_queue(&mut self, queue_name: &str) -> Result<()> {
        self.connection.create_queue(queue_name).await?;
        Ok(())
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
        &self,
        message: &str,
        queue: &str,
        exchange: &str,
        mandatory: bool,
        immediate: bool,
        properties: Properties,
    ) -> Result<()> {
        let mut full_buffer: Vec<u8> = Vec::new();
        let publish = basic::Publish::new(self.channel_id, exchange, queue, mandatory, immediate);
        let bytes = encode_frame(&publish).unwrap();
        full_buffer.extend_from_slice(&bytes);

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
        Ok(())
    }
}
