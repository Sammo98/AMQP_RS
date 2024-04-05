use crate::common::ClientConnection;
use crate::encde::*;
use crate::frame::*;
use std::cell::RefCell;
use std::sync::Arc;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type Handler = Arc<dyn Fn(&[u8]) + Send + Sync>;
pub struct Consumer {
    connection: Arc<ClientConnection>,
    channel: RefCell<u16>,
}

impl Consumer {
    pub async fn new(address: &str) -> Self {
        let connection = Arc::new(ClientConnection::new(address).await);

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
    pub async fn consume_on_queue(&self, queue: &str, handler: Handler) -> Result<()> {
        let consume = basic::Consume::new(ShortString(queue.into()), Bits(vec![0, 0, 0, 0]));
        let bytes = encode_frame(&consume).unwrap();
        self.connection.send(&bytes).await?;

        // ConsumeOk
        let buffer = self.connection.read().await?;
        let _consume_ok: basic::ConsumeOk = decode_frame(&buffer).unwrap();
        println!("Trying to read");

        while let Ok(buffer) = self.connection.read().await {
            println!("Read!");
            let header: Header = decode_frame(&buffer).unwrap();
            let connection = Arc::clone(&self.connection);
            let handler = Arc::clone(&handler);
            if header.frame_type == FrameType::Heartbeat {
                tokio::task::spawn(async move {
                    let heart_beat = [8_u8, 0, 0, 0, 0, 0, 0, 0xCE];
                    println!("Sending heartbeat");
                    connection
                        .send(&heart_beat)
                        .await
                        .expect("Failed to write heartbeat");
                });
                continue;
            } else {
                tokio::task::spawn(async move {
                    let mut frames = buffer.split_inclusive(|&x| x == FRAME_END);
                    let deliver: basic::Deliver = decode_frame(frames.next().unwrap()).unwrap();
                    let content_header: content::Content =
                        decode_frame(frames.next().unwrap()).unwrap();
                    let body: body::BodyReceive = decode_frame(frames.next().unwrap()).unwrap();
                    handler(&body.inner());
                    let ack = basic::Ack::new(deliver.delivery_tag);
                    let bytes = encode_frame(&ack).unwrap();
                    // TODO sort out connection, this lock has contention from the constant polling
                    connection.send(&bytes).await.expect("Failed to ack");
                    println!("Ack sent!");
                });
            }
        }
        Ok(())
    }
}
