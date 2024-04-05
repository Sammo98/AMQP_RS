use crate::{common::*, encde::*, frame::*};
use std::{error::Error, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf},
    net::TcpStream,
    sync::Mutex,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Handler = Arc<dyn Fn(&[u8]) + Send + Sync>;
const FRAME_MAX_SIZE: usize = 4096;

struct TcpConnection {
    reader: ReadHalf<TcpStream>,
    writer: WriteHalf<TcpStream>,
}

impl TcpConnection {
    async fn new(address: &str) -> Self {
        let connection = TcpStream::connect(address)
            .await
            .expect("Failed to connect to address");

        let (reader, writer) = tokio::io::split(connection);

        Self { reader, writer }
    }
    pub async fn write(&mut self, bytes: &[u8]) -> Result<()> {
        self.writer.write_all(bytes).await?;
        Ok(())
    }
    pub async fn read(&mut self) -> Result<[u8; FRAME_MAX_SIZE]> {
        let mut buffer = [0_u8; FRAME_MAX_SIZE];
        self.reader.read(&mut buffer).await?;
        Ok(buffer)
    }
}

pub struct Client {
    pub connection: Arc<Mutex<TcpConnection>>,
    channel: Option<u16>,
}

impl Client {
    pub async fn new(address: &str) -> Self {
        let connection = Arc::new(Mutex::new(TcpConnection::new(address).await));

        Self {
            connection,
            channel: None,
        }
    }

    pub async fn close(&mut self) -> Result<()> {
        let close = connection::Close::new(0, ShortString("a".into()), 10, 50);
        let bytes = encode_frame(&close).unwrap();
        self.connection.lock().await.write(&bytes).await?;
        Ok(())
    }

    pub async fn connect(&mut self) -> Result<()> {
        let protocol_header = connection::ProtocolHeader::new();
        let bytes = encode_frame_static(&protocol_header).unwrap();
        self.connection.lock().await.write(&bytes).await?;

        // Read Start
        let buffer = self.connection.lock().await.read().await?;
        let start: connection::Start = decode_frame(&buffer).unwrap();
        let LongString(locales) = &start.locales;

        // Write StartOk
        let start_ok_test =
            connection::StartOk::new("PLAIN".into(), "\0guest\0guest".into(), locales.clone());
        let bytes = encode_frame(start_ok_test).unwrap();
        self.connection.lock().await.write(&bytes).await?;

        // Read Tune
        let buffer = self.connection.lock().await.read().await?;
        let tune: connection::Tune = decode_frame(&buffer).unwrap();

        // Write TuneOk
        let tune_ok = connection::TuneOk::new(tune.channel_max, tune.frame_max, tune.heartbeat);
        let bytes = encode_frame(tune_ok).unwrap();
        self.connection.lock().await.write(&bytes).await?;

        // Read Open
        let open_test = connection::Open::new("/".into(), "".into(), true);
        let bytes = encode_frame(open_test).unwrap();
        self.connection.lock().await.write(&bytes).await?;

        // OpenOk TODO
        let buffer = self.connection.lock().await.read().await?;
        let _open_ok: connection::OpenOk = decode_frame(&buffer).unwrap();
        Ok(())
    }

    async fn create_channel(&mut self) -> Result<()> {
        match self.channel {
            Some(_) => Ok(()),
            None => {
                let open = channel::Open::new();
                let bytes = encode_frame(&open).unwrap();
                self.connection.lock().await.write(&bytes).await?;

                let buffer = self.connection.lock().await.read().await?;
                let open_ok: channel::OpenOk = decode_frame(&buffer).unwrap();
                self.channel = Some(open_ok.reserved_1); // TODO this is always 0, where do i get the channel
                Ok(())
            }
        }
    }
    pub async fn create_queue(&mut self, queue_name: &str) -> Result<()> {
        if self.channel.is_none() {
            self.create_channel().await?;
        }
        let declare =
            queue::Declare::new(ShortString(queue_name.into()), Bits(vec![0, 0, 0, 0, 0]));
        let bytes = encode_frame(declare).unwrap();
        self.connection.lock().await.write(&bytes).await?;

        let buffer = self.connection.lock().await.read().await?;
        let _declare_ok: queue::DeclareOk = decode_frame(&buffer).unwrap();
        Ok(())
    }

    pub async fn send_message(
        &mut self,
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
        full_buffer.extend_from_slice(&bytes);

        // body
        let mut message_bytes = Vec::new();
        message_bytes.extend_from_slice(message.as_bytes());
        let body = body::Body::new(RawBytes(message_bytes));
        let bytes = encode_frame(&body).unwrap();
        full_buffer.extend_from_slice(&bytes);

        self.connection.lock().await.write(&full_buffer).await?;
        Ok(())
    }

    pub async fn consume_on_queue(&mut self, queue: &str, handler: Handler) -> Result<()> {
        let consume = basic::Consume::new(ShortString(queue.into()), Bits(vec![]));
        let bytes = encode_frame(&consume).unwrap();
        self.connection.lock().await.write(&bytes).await?;

        // ConsumeOk
        let buffer = self.connection.lock().await.read().await?;
        let _consume_ok: basic::ConsumeOk = decode_frame(&buffer).unwrap();

        while let Ok(buffer) = self.connection.lock().await.read().await {
            let header: Header = decode_frame(&buffer).unwrap();
            let connection = Arc::clone(&self.connection);
            let handler = Arc::clone(&handler);
            if header.frame_type == FrameType::Heartbeat {
                tokio::task::spawn(async move {
                    let heart_beat = [8_u8, 0, 0, 0, 0, 0, 0, 0xCE];
                    println!("Sending heartbeat");
                    connection
                        .lock()
                        .await
                        .write(&heart_beat)
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
                    connection
                        .lock()
                        .await
                        .write(&bytes)
                        .await
                        .expect("Failed to ack");
                });
            }
        }
        Ok(())
    }
}
