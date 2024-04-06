use crate::encde::*;
use crate::frame::*;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf},
    net::TcpStream,
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct AdapterReader {
    tcp_reader: ReadHalf<TcpStream>,
    sender: UnboundedSender<Vec<u8>>,
}

impl AdapterReader {
    pub async fn start(&mut self) {
        loop {
            let mut buffer = [0_u8; 1024];
            match self.tcp_reader.read(&mut buffer).await {
                Ok(_) => {
                    if let Err(e) = self.sender.send(buffer.to_vec()) {
                        println!("Error sending to Client {:?}", e);
                    }
                    continue;
                }
                Err(_) => todo!(),
            }
        }
    }
}
struct AdapterWriter {
    tcp_writer: WriteHalf<TcpStream>,
    receiver: UnboundedReceiver<Vec<u8>>,
}

impl AdapterWriter {
    pub async fn start(&mut self) {
        while let Some(bytes) = self.receiver.recv().await {
            self.tcp_writer.write_all(&bytes).await.unwrap();
        }
    }
}
pub struct TcpAdapter {
    tcp_sender: UnboundedSender<Vec<u8>>,
    tcp_receiver: UnboundedReceiver<Vec<u8>>,
}

impl TcpAdapter {
    pub fn clone_sender(&self) -> UnboundedSender<Vec<u8>> {
        self.tcp_sender.clone()
    }
    pub async fn new(address: &str) -> Self {
        let stream = TcpStream::connect(address)
            .await
            .expect("Failed to create TCP connection. Is RabbitMQ running?");
        let (tcp_reader, tcp_writer) = tokio::io::split(stream);

        let (tcp_sender, receiver): (UnboundedSender<Vec<u8>>, UnboundedReceiver<Vec<u8>>) =
            mpsc::unbounded_channel();

        let mut adapter_writer = AdapterWriter {
            tcp_writer,
            receiver,
        };

        let (sender, tcp_receiver): (UnboundedSender<Vec<u8>>, UnboundedReceiver<Vec<u8>>) =
            mpsc::unbounded_channel();
        let mut adapter_reader = AdapterReader { tcp_reader, sender };

        tokio::task::spawn(async move {
            adapter_writer.start().await;
        });
        tokio::task::spawn(async move {
            adapter_reader.start().await;
        });

        Self {
            tcp_sender,
            tcp_receiver,
        }
    }

    pub async fn send(&self, bytes: Vec<u8>) {
        let _x = self.tcp_sender.send(bytes);
    }

    pub async fn receive(&mut self) -> Option<Vec<u8>> {
        self.tcp_receiver.recv().await
    }
    pub async fn connect(&mut self) -> Result<()> {
        // Break this up
        let protocol_header = connection::ProtocolHeader::new();
        let bytes = encode_frame_static(&protocol_header).unwrap();
        self.send(bytes).await;

        // Read Start
        let buffer = self.receive().await.unwrap();
        let start: connection::Start = decode_frame(&buffer).unwrap();
        let LongString(locales) = &start.locales;

        // Write StartOk
        let start_ok_test =
            connection::StartOk::new("PLAIN".into(), "\0guest\0guest".into(), locales.clone());
        let bytes = encode_frame(start_ok_test).unwrap();
        self.send(bytes).await;

        // Read Tune
        let buffer = self.receive().await.unwrap();
        let tune: connection::Tune = decode_frame(&buffer).unwrap();

        // Write TuneOk
        let tune_ok = connection::TuneOk::new(tune.channel_max, tune.frame_max, tune.heartbeat);
        let bytes = encode_frame(tune_ok).unwrap();
        self.send(bytes).await;

        // Read Open
        let open_test = connection::Open::new("/".into(), "".into(), true);
        let bytes = encode_frame(open_test).unwrap();
        self.send(bytes).await;
        // OpenOk
        let buffer = self.receive().await.unwrap();
        let _open_ok: connection::OpenOk = decode_frame(&buffer).unwrap();
        Ok(())
    }

    pub async fn create_channel(&mut self) -> Result<u16> {
        let open = channel::Open::new();
        let bytes = encode_frame(&open).unwrap();
        self.send(bytes).await;

        let buffer = self.receive().await.unwrap();
        let _open_ok: channel::OpenOk = decode_frame(&buffer).unwrap();
        Ok(1)
    }
    pub async fn create_queue(&mut self, queue_name: &str) -> Result<()> {
        let declare =
            queue::Declare::new(ShortString(queue_name.into()), Bits(vec![0, 0, 0, 0, 0]));
        let bytes = encode_frame(declare).unwrap();
        self.send(bytes).await;

        let buffer = self.receive().await.unwrap();
        let declare_ok: queue::DeclareOk = decode_frame(&buffer).unwrap();
        println!(
            "Pre existing messages on {queue_name}: {}",
            declare_ok.message_count
        );
        Ok(())
    }
}
