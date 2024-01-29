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
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct Client {
    stream: TcpStream,
    max_buffer_size: usize,
    channel: Option<u16>,
}

impl Client {
    pub fn new(address: &str) -> Self {
        let stream = TcpStream::connect(address).expect("Failed to connect to address");
        Self {
            stream,
            max_buffer_size: FRAME_MAX_SIZE,
            channel: None,
        }
    }

    fn read(&mut self) -> Result<[u8; FRAME_MAX_SIZE]> {
        let mut buffer = [0_u8; FRAME_MAX_SIZE];
        self.stream.read(&mut buffer)?;
        Ok(buffer)
    }

    pub fn connect(&mut self) -> Result<()> {
        // Write protocol header
        _ = self.stream.write(&PROTOCOL_HEADER)?;

        // Read Start
        let start: connection::Start;
        {
            let buffer = self.read()?;
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
            self.stream.write(&start_ok)?;
        }

        // Read Tune
        let tune: connection::Tune;
        {
            let buffer = self.read()?;
            tune = connection::Tune::from_frame(&buffer);
        }

        // Write TuneOk
        let tune_ok: Vec<u8>;
        {
            tune_ok =
                connection::TuneOk::to_frame(tune.channel_max, tune.frame_max, tune.heartbeat);
            self.stream.write(&tune_ok)?;
        }

        // Read Open
        let open: Vec<u8>;
        {
            open = connection::Open::to_frame("/".into(), "".into(), true);
            self.stream.write(&open)?;
        }

        // OpenOk TODO
        _ = self.read()?;

        Ok(())
    }

    fn create_channel(&mut self) -> Result<()> {
        match self.channel {
            Some(_) => Ok(()),
            None => {
                let open: Vec<u8> = channel::Open::to_frame();
                self.stream.write(&open)?;
                let buffer = self.read()?;
                let open_ok = channel::OpenOk::from_frame(&buffer);
                self.channel = Some(open_ok.channel);
                Ok(())
            }
        }
    }
    pub fn create_queue(&mut self, queue_name: &str) -> Result<()> {
        if let None = self.channel {
            self.create_channel()?;
        }

        let declare = queue::Declare::to_frame(queue_name, false, false, false, false, false);
        self.stream.write(&declare)?;

        // DeclareOk TODO
        _ = self.read()?;

        Ok(())
    }

    pub fn send_message(
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

        self.stream.write(&full_buffer)?;
        Ok(())

        // Body
    }

    pub fn consume_on_queue(&mut self, queue: &str) -> Result<()> {
        let consume = basic::Consume::to_frame(queue);
        self.stream.write(&consume)?;

        // Consume okay!
        let mut buffer = [0_u8; 1000];
        self.stream.read(&mut buffer)?;

        let mut buffer = [0_u8; FRAME_MAX_SIZE];
        while let Ok(_) = self.stream.read(&mut buffer) {
            let mut decoder = Decoder::new(&buffer);
            let _header = decoder.take_header();
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
            let header = decoder.take_header();
            let x = decoder.take_till_frame_end();
            let y = String::from_utf8(x);
            println!("RECEIVED MESSAGE {y:?}");
            buffer = [0_u8; FRAME_MAX_SIZE];
        }
        Ok(())
    }
}
