use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf},
    net::TcpStream,
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
};

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
        self.tcp_sender.send(bytes).unwrap();
    }

    pub async fn receive(&mut self) -> Option<Vec<u8>> {
        self.tcp_receiver.recv().await
    }
}
