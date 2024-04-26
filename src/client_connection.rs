use std::sync::atomic::AtomicU16;

use tokio::sync::mpsc::UnboundedSender;

use crate::encde::*;
use crate::frame::*;
use crate::tcp::TcpAdapter;
use crate::types::*;

fn get_channel_id() -> u16 {
    static ID_COUNTER: AtomicU16 = AtomicU16::new(1);
    ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

pub struct Connection {
    tcp_adapter: TcpAdapter,
    pub channel_id: u16,
}

impl Connection {
    pub fn get_writer(&self) -> UnboundedSender<Vec<u8>> {
        self.tcp_adapter.clone_sender()
    }
    pub async fn connect(address: &str) -> Self {
        let mut tcp_adapter = TcpAdapter::new(address).await;

        let protocol_header = connection::ProtocolHeader::new();
        let bytes = encode_frame_static(&protocol_header).unwrap();
        tcp_adapter.send(bytes).await;

        // Read Start
        let buffer = tcp_adapter.receive().await.unwrap();
        let start: connection::Start = decode_frame(&buffer).unwrap();

        // Write StartOk
        let start_ok_test = connection::StartOk::new("PLAIN", "\0guest\0guest", &start.locales);
        let bytes = encode_frame(start_ok_test).unwrap();
        tcp_adapter.send(bytes).await;

        // Read Tune
        let buffer = tcp_adapter.receive().await.unwrap();
        let tune: connection::Tune = decode_frame(&buffer).unwrap();

        // Write TuneOk
        let tune_ok = connection::TuneOk::new(tune.channel_max, tune.frame_max, tune.heartbeat);
        let bytes = encode_frame(tune_ok).unwrap();
        tcp_adapter.send(bytes).await;

        // Read Open
        let open_test = connection::Open::new("/");
        let bytes = encode_frame(open_test).unwrap();
        tcp_adapter.send(bytes).await;
        // OpenOk
        let buffer = tcp_adapter.receive().await.unwrap();
        let _open_ok: connection::OpenOk = decode_frame(&buffer).unwrap();
        Self {
            tcp_adapter,
            channel_id: get_channel_id(),
        }
    }

    pub async fn create_channel(&mut self) -> Result<u16> {
        let open = channel::Open::new(self.channel_id);
        let bytes = encode_frame(&open).unwrap();
        self.write(bytes).await;

        let buffer = self.read().await.unwrap();
        let _open_ok: channel::OpenOk = decode_frame(&buffer).unwrap();
        Ok(1)
    }
    pub async fn create_queue(&mut self, queue_name: &str) -> Result<()> {
        let declare = queue::Declare::new(
            self.channel_id,
            queue_name,
            false,
            false,
            false,
            false,
            false,
        );
        let bytes = encode_frame(declare).unwrap();
        self.write(bytes).await;

        let buffer = self.read().await.unwrap();
        let _declare_ok: queue::DeclareOk = decode_frame(&buffer).unwrap();
        Ok(())
    }

    pub async fn create_exchange(
        &mut self,
        exchange: &str,
        exchange_type: ExchangeType,
    ) -> Result<()> {
        let declare = exchange::Declare::new(self.channel_id, exchange.into(), exchange_type);
        let bytes = encode_frame(declare)?;
        self.write(bytes).await;

        let buffer = self.read().await.unwrap();
        let _declare_ok: exchange::DeclareOk = decode_frame(&buffer).unwrap();
        Ok(())
    }

    pub async fn delete_exchange(&mut self, exchange: &str) -> Result<()> {
        let delete = exchange::Delete::new(self.channel_id, exchange);
        let bytes = encode_frame(delete)?;
        self.write(bytes).await;

        let buffer = self.read().await.unwrap();
        let _declare_ok: exchange::DeleteOk = decode_frame(&buffer).unwrap();
        Ok(())
    }

    pub async fn write(&self, bytes: Vec<u8>) {
        self.tcp_adapter.send(bytes).await;
    }
    pub async fn read(&mut self) -> Option<Vec<u8>> {
        self.tcp_adapter.receive().await
    }
}
