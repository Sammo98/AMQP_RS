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
        // Write protocol headero
        println!("Trying to write prot head");
        let x = self.connection.lock().await.write(&PROTOCOL_HEADER).await?;
        println!("{x:?}");
        println!("Wrote protocol header");

        // Read Start
        let start: connection::Start;
        {
            println!("Trying to read constart");
            let buffer = self.connection.lock().await.read().await?;
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
            self.connection.lock().await.write(&start_ok).await?;
            println!("Wrote start ok ");
        }

        // Read Tune
        let tune: connection::Tune;
        {
            let buffer = self.connection.lock().await.read().await?;
            println!("Read tune");
            tune = connection::Tune::from_frame(&buffer);
        }

        // Write TuneOk
        let tune_ok: Vec<u8>;
        {
            tune_ok =
                connection::TuneOk::to_frame(tune.channel_max, tune.frame_max, tune.heartbeat);
            self.connection.lock().await.write(&tune_ok).await?;
        }

        // Read Open
        let open: Vec<u8>;
        {
            open = connection::Open::to_frame("/".into(), "".into(), true);
            self.connection.lock().await.write(&open).await?;
        }

        // OpenOk TODO
        _ = self.connection.lock().await.read().await?;

        Ok(())
    }

    async fn create_channel(&mut self) -> Result<()> {
        match self.channel {
            Some(_) => Ok(()),
            None => {
                let open: Vec<u8> = channel::Open::to_frame();
                self.connection.lock().await.write(&open).await?;
                let buffer = self.connection.lock().await.read().await?;
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
        self.connection.lock().await.write(&declare).await?;

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

        self.connection.lock().await.write(&full_buffer).await?;
        Ok(())

        // Body
    }

    pub async fn consume_on_queue(&mut self, queue: &str, handler: Handler) -> Result<()> {
        let consume = basic::Consume::to_frame(queue);
        self.connection.lock().await.write(&consume).await?;

        // Consume okay!
        self.connection.lock().await.read().await?;

        while let Ok(buffer) = self.connection.lock().await.read().await {
            let connection = Arc::clone(&self.connection);
            let mut decoder = Decoder::new(&buffer);
            let header = decoder.take_header();
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
                    let mut decoder = Decoder::new(&buffer);
                    let _ = decoder.take_header();
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
                    let _properties = if (flags & crate::constants::properties::HEADERS) != 0 {
                        decoder.take_table()
                    } else {
                        HashMap::new()
                    };
                    let _properties = if (flags & crate::constants::properties::DELIVERY_MODE) != 0 {
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
                    let string = String::from_utf8(body).expect("Failed");
                    handler(string);
                });
            }
            // let _class = decoder.take_class_type();
            // let _method = decoder.take_method_type();
            // let _consumer_tag = decoder.take_short_string();
            // let _delivery_tag = decoder.take_u64();
            // let _redelivered = decoder.take_bool();
            // let _exchange = decoder.take_short_string();
            // let _routing = decoder.take_short_string();
            // decoder.next_frame();
            // let _header = decoder.take_header();
            // let _class = decoder.take_class_type();
            // let _weight = decoder.take_u16();
            // let _length = decoder.take_u64();

            // // properties - I need to learn how this works, but this is following pika logic
            // let mut flags = 0_u64;
            // let mut flag_index = 0_u16;
            // loop {
            //     let partial_flags = decoder.take_u16() as u64;
            //     flags = flags | (partial_flags << (flag_index * 16));
            //     if (partial_flags & 1) == 0 {
            //         break;
            //     } else {
            //         flag_index += 1;
            //     }
            // }
            // let properties = if (flags & crate::constants::properties::HEADERS) != 0 {
            //     decoder.take_table()
            // } else {
            //     HashMap::new()
            // };
            // let properties = if (flags & crate::constants::properties::DELIVERY_MODE) != 0 {
            //     println!("we got a delivery mode!");
            //     decoder.take_u8()
            // } else {
            //     0
            // };
            // decoder.next_frame();
            // // Header contains siez
            // let header = decoder.take_header();
            // println!("{header:?}");

            // let body = decoder.take_till_frame_end();
            // let string = String::from_utf8(body)?;
            // handler(string);
        }
        Ok(())
    }
}
