use crate::body::Body;
use crate::body::BodyReceive;
use crate::common::FrameType;
use crate::common::Header;
use crate::constants::size::FRAME_MAX_SIZE;
use crate::constants::CONFIG;
use crate::constants::PROTOCOL_HEADER;
use crate::content::Content;
use crate::endec::RawBytes;
use crate::endec::{Bits, LongString, ShortString};
use crate::method::basic;
use crate::method::basic::Consume;
use crate::method::channel;
use crate::method::connection;
use crate::method::queue;
use bincode;
use std::error::Error;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::ReadHalf;
use tokio::io::WriteHalf;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Handler = Arc<dyn Fn(String) + Send + Sync>;

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
    connection: Arc<Mutex<TcpConnection>>,
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

    pub async fn connect(&mut self) -> Result<()> {
        _ = self.connection.lock().await.write(&PROTOCOL_HEADER).await?;

        // Read Start
        let buffer = self.connection.lock().await.read().await?;
        let (start, _): (connection::Start, usize) =
            bincode::decode_from_slice(&buffer, CONFIG).unwrap();

        let LongString(locales) = &start.locales;
        // Write StartOk
        let start_ok_test =
            connection::StartOk::new("PLAIN".into(), "\0guest\0guest".into(), locales.clone());
        let mut bytes = bincode::encode_to_vec(&start_ok_test, CONFIG).unwrap();
        let frame_length = ((bytes.len() - 8) as u32).to_be_bytes();
        bytes.splice(3..7, frame_length);

        self.connection.lock().await.write(&bytes).await?;

        // Read Tune
        let buffer = self.connection.lock().await.read().await?;
        let (tune, _): (connection::Tune, usize) =
            bincode::decode_from_slice(&buffer, CONFIG).unwrap();

        // Write TuneOk
        let tune_ok = connection::TuneOk::new(tune.channel_max, tune.frame_max, tune.heartbeat);
        let mut bytes = bincode::encode_to_vec(&tune_ok, CONFIG).unwrap();
        // 8 is the length of the frame excluding the header and the frame end.
        let frame_length = ((bytes.len() - 8) as u32).to_be_bytes();
        bytes.splice(3..7, frame_length);
        self.connection.lock().await.write(&bytes).await?;

        // Read Open
        let open_test = connection::Open::new("/".into(), "".into(), true);
        let mut bytes = bincode::encode_to_vec(&open_test, CONFIG).unwrap();
        let frame_length = ((bytes.len() - 8) as u32).to_be_bytes();
        bytes.splice(3..7, frame_length);
        self.connection.lock().await.write(&bytes).await?;

        // OpenOk TODO
        _ = self.connection.lock().await.read().await?;

        Ok(())
    }

    async fn create_channel(&mut self) -> Result<()> {
        match self.channel {
            Some(_) => Ok(()),
            None => {
                let open = channel::Open::new();
                let mut bytes = bincode::encode_to_vec(&open, CONFIG).unwrap();
                let frame_length = ((bytes.len() - 8) as u32).to_be_bytes();
                bytes.splice(3..7, frame_length);
                self.connection.lock().await.write(&bytes).await?;

                let buffer = self.connection.lock().await.read().await?;
                let (open_ok, _): (channel::OpenOk, usize) =
                    bincode::decode_from_slice(&buffer, CONFIG).unwrap();
                self.channel = Some(open_ok.reserved_1);
                Ok(())
            }
        }
    }
    pub async fn create_queue(&mut self, queue_name: &str) -> Result<()> {
        if let None = self.channel {
            self.create_channel().await?;
        }
        let declare =
            queue::Declare::new(ShortString(queue_name.into()), Bits(vec![0, 0, 0, 0, 0]));
        let mut bytes = bincode::encode_to_vec(&declare, CONFIG).unwrap();
        let frame_length = ((bytes.len() - 8) as u32).to_be_bytes();
        bytes.splice(3..7, frame_length);

        self.connection.lock().await.write(&bytes).await?;

        // DeclareOk TODO
        _ = self.connection.lock().await.read().await?;

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
        let publish_test = basic::Publish::new(
            ShortString(exchange.into()),
            ShortString(queue.into()),
            Bits(vec![mandatory as u8, immediate as u8]),
        );
        let mut bytes = bincode::encode_to_vec(&publish_test, CONFIG).unwrap();
        let frame_length = ((bytes.len() - 8) as u32).to_be_bytes();
        bytes.splice(3..7, frame_length);
        full_buffer.extend_from_slice(&bytes);

        // Content header
        let content_header = Content::new(message.len() as u64);
        let mut bytes = bincode::encode_to_vec(&content_header, CONFIG).unwrap();
        let frame_length = ((bytes.len() - 8) as u32).to_be_bytes();
        bytes.splice(3..7, frame_length);
        full_buffer.extend_from_slice(&bytes);

        // body
        let mut test = Vec::new();
        test.extend_from_slice(message.as_bytes());
        let body_test = Body::new(RawBytes(test));
        let mut bytes = bincode::encode_to_vec(&body_test, CONFIG).unwrap();
        let frame_length = ((bytes.len() - 8) as u32).to_be_bytes();
        bytes.splice(3..7, frame_length);
        full_buffer.extend_from_slice(&bytes);

        self.connection.lock().await.write(&full_buffer).await?;
        Ok(())

        // Body
    }

    pub async fn consume_on_queue(&mut self, queue: &str, handler: Handler) -> Result<()> {
        let test = Consume::new(
            ShortString(queue.into()),
            ShortString("CONSUMER_TAG2".into()),
            Bits(vec![]),
        );
        let mut bytes = bincode::encode_to_vec(&test, CONFIG).unwrap();
        let frame_length = ((bytes.len() - 8) as u32).to_be_bytes();
        bytes.splice(3..7, frame_length);
        self.connection.lock().await.write(&bytes).await?;

        // Consume okay!
        self.connection.lock().await.read().await?;

        while let Ok(buffer) = self.connection.lock().await.read().await {
            let (header, _): (Header, usize) = bincode::decode_from_slice(&buffer, CONFIG).unwrap();
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
                    let mut test = buffer.split_inclusive(|&x| x == 206);
                    let (deliver, _): (basic::Deliver, usize) =
                        bincode::decode_from_slice(test.next().unwrap(), CONFIG).unwrap();
                    let (content_header, _): (Content, usize) =
                        bincode::decode_from_slice(test.next().unwrap(), CONFIG).unwrap();
                    let (body, _): (BodyReceive, usize) =
                        bincode::decode_from_slice(test.next().unwrap(), CONFIG).unwrap();
                    let message = body
                        .content
                        .iter()
                        .cloned()
                        .filter(|&x| x != 0xCE)
                        .collect::<Vec<u8>>();
                    let string = String::from_utf8(message).expect("Failed");
                    handler(string);
                });
            }
        }
        Ok(())
    }
}
