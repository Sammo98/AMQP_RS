use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::encde::*;
use crate::frame::*;
use crate::tcp::TcpAdapter;
use std::cell::RefCell;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type Handler = &'static (dyn Fn(&[u8]) + Send + Sync);
pub struct Consumer {
    tcp_adapter: TcpAdapter,
    channel: RefCell<u16>,
}

impl Consumer {
    pub async fn new(address: &str) -> Self {
        let tcp_adapter = TcpAdapter::new(address).await; // This should automatically call connect, otherwise the handshake might timeout

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
        self.tcp_adapter.create_channel().await?;
        Ok(())
    }

    pub async fn create_queue(&mut self, queue_name: &str) -> Result<()> {
        self.tcp_adapter.create_queue(queue_name).await?;
        Ok(())
    }
    pub async fn consume_on_queue(&mut self, queue: &str, handler: Handler) -> Result<()> {
        let consume = basic::Consume::new(ShortString(queue.into()), Bits(vec![0, 0, 0, 0]));
        let bytes = encode_frame(&consume).unwrap();
        self.tcp_adapter.send(bytes).await;

        // ConsumeOk
        let buffer = self.tcp_adapter.receive().await.unwrap();
        let _consume_ok: basic::ConsumeOk = decode_frame(&buffer).unwrap();
        let (tx, rx) = mpsc::unbounded_channel();
        tokio::task::spawn(async move { consumer_task(rx, handler).await });

        while let Some(buffer) = self.tcp_adapter.receive().await {
            let header: Header = decode_frame(&buffer).unwrap();
            if header.frame_type == FrameType::Heartbeat {
                let heart_beat = [8_u8, 0, 0, 0, 0, 0, 0, 0xCE].to_vec();
                self.tcp_adapter.send(heart_beat).await;
                println!("Sent heartbeat");
                continue;
            } else {
                let mut frames = buffer.split_inclusive(|&x| x == FRAME_END);
                let deliver: basic::Deliver = decode_frame(frames.next().unwrap()).unwrap();
                let content_header: content::Content =
                    decode_frame(frames.next().unwrap()).unwrap();
                let body: body::BodyReceive = decode_frame(frames.next().unwrap()).unwrap();
                let bytes = body.inner();
                let _x = tx.send(bytes);
            }
        }
        Ok(())
    }
}

async fn consumer_task(mut receiver: UnboundedReceiver<Vec<u8>>, handler: Handler) {
    println!("Consumer started");
    while let Some(message) = receiver.recv().await {
        tokio::task::spawn(async move {
            handler(&message);
        });
    }
}
