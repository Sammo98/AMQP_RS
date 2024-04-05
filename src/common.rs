use crate::{encde::*, frame::*};
use std::{error::Error, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf},
    net::TcpStream,
    sync::Mutex,
};
const FRAME_MAX_SIZE: usize = 4096; // Todo make this a parameter

type Result<T> = std::result::Result<T, Box<dyn Error>>;
pub struct ClientConnection {
    connection: Arc<Mutex<TcpConnection>>,
}

impl ClientConnection {
    pub async fn new(address: &str) -> Self {
        let connection = Arc::new(Mutex::new(TcpConnection::new(address).await));

        Self { connection }
    }

    pub async fn send(&self, bytes: &[u8]) -> Result<()> {
        self.connection.lock().await.write(bytes).await?;
        Ok(())
    }

    pub async fn read(&self) -> Result<[u8; 1024]> {
        let bytes = self.connection.lock().await.read().await?;
        Ok(bytes)
    }

    pub async fn connect(&self) -> Result<()> {
        // Break this up
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

        // OpenOk
        let buffer = self.connection.lock().await.read().await?;
        let _open_ok: connection::OpenOk = decode_frame(&buffer).unwrap();
        Ok(())
    }

    pub async fn create_channel(&self) -> Result<u16> {
        let open = channel::Open::new();
        let bytes = encode_frame(&open).unwrap();
        self.connection.lock().await.write(&bytes).await?;

        let buffer = self.connection.lock().await.read().await?;
        let _open_ok: channel::OpenOk = decode_frame(&buffer).unwrap();
        Ok(1)
    }

    pub async fn create_queue(&self, queue_name: &str) -> Result<()> {
        let declare =
            queue::Declare::new(ShortString(queue_name.into()), Bits(vec![0, 0, 0, 0, 0]));
        let bytes = encode_frame(declare).unwrap();
        self.connection.lock().await.write(&bytes).await?;

        let buffer = self.connection.lock().await.read().await?;
        let declare_ok: queue::DeclareOk = decode_frame(&buffer).unwrap();
        println!(
            "Pre existing messages on {queue_name}: {}",
            declare_ok.message_count
        );
        Ok(())
    }
}

struct TcpConnection {
    pub connection: TcpStream,
}

impl TcpConnection {
    async fn new(address: &str) -> Self {
        let connection = TcpStream::connect(address)
            .await
            .expect("Failed to connect to address");

        Self { connection }
    }
    pub async fn write(&mut self, bytes: &[u8]) -> Result<()> {
        self.connection.write_all(bytes).await?;
        Ok(())
    }
    pub async fn read(&mut self) -> Result<[u8; 1024]> {
        let mut buf = [0_u8; 1024];
        loop {
            let bytes_read = self.connection.read(&mut buf).await?;
            match buf[bytes_read - 1] {
                0xCE | 0 => break,
                _ => continue,
            }
        }
        Ok(buf)
    }
}
