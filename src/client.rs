use crate::common::FrameType;
use crate::communication::decode::Decoder;
use crate::communication::encode::Encoder;
use crate::constants::class_id;
use crate::constants::frame_type;
use crate::constants::size::FRAME_MAX_SIZE;
use crate::constants::PROTOCOL_HEADER;
use crate::method::basic;
use crate::method::channel;
use crate::method::connection;
use crate::method::queue;
use std::collections::HashMap;
use std::error::Error;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Handler = fn(String);

pub struct Client {
    connection: TcpStream,
    max_buffer_size: usize,
    channel: Option<u16>,
}

impl Client {
    pub async fn new(address: &str) -> Self {
        let connection = TcpStream::connect(address)
            .await
            .expect("Failed to connect to address");

        Self {
            connection,
            max_buffer_size: FRAME_MAX_SIZE,
            channel: None,
        }
    }

    async fn read(&mut self) -> Result<[u8; FRAME_MAX_SIZE]> {
        let mut buffer = [0_u8; FRAME_MAX_SIZE];
        self.connection.read(&mut buffer).await?;
        Ok(buffer)
    }

    async fn write(&mut self, bytes: &[u8]) -> Result<()> {
        self.connection.write_all(bytes).await?;
        Ok(())
    }

    pub async fn connect(&mut self) -> Result<()> {
        // Write protocol headero
        println!("Trying to write prot head");
        let x = self.write(&PROTOCOL_HEADER).await;
        println!("{x:?}");
        println!("Wrote protocol header");

        // Read Start
        let start: connection::Start;
        {
            println!("Trying to read constart");
            let buffer = self.read().await?;
            start = connection::Start::from_frame(&buffer);
        }

        // Write StartOk
        let start_ok: Vec<u8>;
        {
            start_ok = connection::StartOk::to_frame(
                "PLAIN".into(),
                "\0guest\0guest".into(),
                start.locales,
            );
            self.write(&start_ok).await?;
            println!("Wrote start ok ");
        }

        // Read Tune
        let tune: connection::Tune;
        {
            let buffer = self.read().await?;
            println!("Read tune");
            tune = connection::Tune::from_frame(&buffer);
        }

        // Write TuneOk
        let tune_ok: Vec<u8>;
        {
            tune_ok =
                connection::TuneOk::to_frame(tune.channel_max, tune.frame_max, tune.heartbeat);
            self.write(&tune_ok).await?;
        }

        // Read Open
        let open: Vec<u8>;
        {
            open = connection::Open::to_frame("/".into(), "".into(), true);
            self.write(&open).await?;
        }

        // OpenOk TODO
        _ = self.read().await?;

        Ok(())
    }

    async fn create_channel(&mut self) -> Result<()> {
        match self.channel {
            Some(_) => Ok(()),
            None => {
                let open: Vec<u8> = channel::Open::to_frame();
                self.write(&open).await?;
                let buffer = self.read().await?;
                let open_ok = channel::OpenOk::from_frame(&buffer);
                self.channel = Some(open_ok.channel);
                Ok(())
            }
        }
    }
    pub async fn create_queue(&mut self, queue_name: &str) -> Result<()> {
        if let None = self.channel {
            self.create_channel().await?;
        }

        let declare = queue::Declare::to_frame(queue_name, false, false, false, false, false);
        self.write(&declare).await?;

        // DeclareOk TODO
        _ = self.read().await?;

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
        let publish = basic::Publish::to_frame(queue, exchange, mandatory, immediate);
        full_buffer.extend_from_slice(&publish);

        // Content header
        let encoder = Encoder::new();
        let content_header = encoder.build_content_frame(
            frame_type::HEADER,
            class_id::BASIC,
            self.channel.unwrap(),
            message.len() as u64,
        );
        full_buffer.extend_from_slice(&content_header);

        // body
        let encoder = Encoder::new();
        let body = encoder.build_body_frame(self.channel.unwrap(), message.into());
        full_buffer.extend_from_slice(&body);
        println!("{full_buffer:?}");

        self.write(&full_buffer).await?;
        Ok(())

        // Body
    }

    pub async fn consume_on_queue(&mut self, queue: &str, handler: impl Fn(String)) -> Result<()> {
        let consume = basic::Consume::to_frame(queue);
        self.write(&consume).await?;

        // Consume okay!
        self.read().await?;

        while let Ok(buffer) = self.read().await {
            let mut decoder = Decoder::new(&buffer);
            let header = decoder.take_header();
            if header.frame_type == FrameType::Heartbeat {
                let heart_beat = [8_u8, 0, 0, 0, 0, 0, 0, 0xCE];
                self.write(&heart_beat).await?;
                continue;
            }
            println!("Header is: {header:?}");
            let _class = decoder.take_class_type();
            let _method = decoder.take_method_type();
            let _consumer_tag = decoder.take_short_string();
            let _delivery_tag = decoder.take_u64();
            let _redelivered = decoder.take_bool();
            let _exchange = decoder.take_short_string();
            let _routing = decoder.take_short_string();
            decoder.next_frame();
            let _header = decoder.take_header();
            let _class = decoder.take_class_type();
            let _weight = decoder.take_u16();
            let _length = decoder.take_u64();

            // properties - I need to learn how this works, but this is following pika logic
            let mut flags = 0_u64;
            let mut flag_index = 0_u16;
            loop {
                let partial_flags = decoder.take_u16() as u64;
                flags = flags | (partial_flags << (flag_index * 16));
                if (partial_flags & 1) == 0 {
                    break;
                } else {
                    flag_index += 1;
                }
            }
            let properties = if (flags & crate::constants::properties::HEADERS) != 0 {
                decoder.take_table()
            } else {
                HashMap::new()
            };
            let properties = if (flags & crate::constants::properties::DELIVERY_MODE) != 0 {
                println!("we got a delivery mode!");
                decoder.take_u8()
            } else {
                0
            };
            decoder.next_frame();
            // Header contains siez
            let header = decoder.take_header();
            println!("{header:?}");

            let body = decoder.take_till_frame_end();
            let string = String::from_utf8(body)?;
            handler(string);
        }
        Ok(())
    }
}
