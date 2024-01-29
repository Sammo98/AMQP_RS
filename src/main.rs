use core::panic;
use std::char::DecodeUtf16;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
use std::rc::Rc;

mod client;
mod common;
mod communication;
mod constants;
mod errors;
mod method;
mod properties;

use method::basic;
use method::channel;
use method::connection::{Close, Open, ProtocolHeader, Start, StartOk, Tune, TuneOk};
use method::queue;

use crate::communication::decode::Decoder;
use crate::communication::encode::Encoder;
use crate::communication::{decode, encode, Value};
use crate::constants::class_id;
use crate::constants::connection_method_id::START;
use crate::constants::frame_type;

use common::ClassType;
use common::ConnectionClassMethodType;
use common::FrameType;
use common::Header;
use constants::{size, PROTOCOL_HEADER};

use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
struct Client {
    stream: TcpStream,
    version_major: u8,
    version_minor: u8,
}

impl Client {
    fn write(&mut self, buffer: &[u8]) {
        self.stream.write(&buffer).unwrap();
    }

    fn read(&mut self) -> [u8; size::FRAME_MAX_SIZE] {
        let mut buffer = [0; size::FRAME_MAX_SIZE];
        // match stream.try_read(&mut data) {
        //     Ok(n) => {
        //         println!("read {} bytes", n);
        //     }
        //     Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
        //         continue;
        //     }
        //     Err(e) => {
        //         return Err(e.into());
        //     }
        // }
        buffer
    }

    fn connect(&mut self) {
        self.write(&ProtocolHeader::to_frame());

        let start_buffer = self.read(); // Connection.Start
        let start = Start::from_frame(&start_buffer);
        let start_ok = StartOk::to_frame("PLAIN".into(), "\0guest\0guest".into(), start.locales);
        self.write(&start_ok);

        let tune_buffer = self.read();
        let tune = Tune::from_frame(&tune_buffer);

        // TUNE OK
        let tune_ok = TuneOk::to_frame(tune.channel_max, tune.frame_max, tune.heartbeat);
        self.write(&tune_ok);

        let open = Open::to_frame("/".into(), "".into(), true);
        self.write(&open);

        // Open ok ?
        _ = self.read();

        let open_channel = channel::Open::to_frame();
        self.write(&open_channel);

        let _open_okay = channel::OpenOk::from_frame(&self.read());

        let declare = queue::Declare::to_frame("my_queue", false, false, false, false, false);
        self.write(&declare);
        _ = self.read();

        // let mut message_vec: Vec<u8> = Vec::new();
        // let publish = basic::Publish::to_frame("my_queue", "", false, false);

        // message_vec.extend_from_slice(&publish);

        // let encoder = Encoder::new();
        // let f = encoder.build_content_frame(frame_type::HEADER, class_id::BASIC, 1, 12);
        // message_vec.extend_from_slice(&f);
        // // self.write(&f);

        // let encoder = Encoder::new();
        // let b = encoder.build_body_frame(1, "Hello World!".into());
        // message_vec.extend_from_slice(&b);
        // self.write(&message_vec);
        // self.write(&b);

        let consume = basic::Consume::to_frame("my_queue");
        self.write(&consume);

        // This would be consume ok
        _ = self.read();

        // Here we are waiting on messages
        let result = self.read();
        println!("After read");

        // This code encapsulates the Basic.Deliver frame
        let mut dec = Decoder::new(&result);
        let header = dec.take_header();
        let class = dec.take_class_type();
        let _ = dec.take_method_type();
        let consumer_tag = dec.take_short_string();
        let delivery_tag = dec.take_u64();
        let redelivered = dec.take_bool();
        let exchange = dec.take_short_string();
        let routing = dec.take_short_string();

        // Content frame?
        dec.next_frame();
        let header = dec.take_header();
        println!("{header:?}");
        let class = dec.take_class_type();
        println!("{class:?}");
        let _weight = dec.take_u16();
        let length = dec.take_u64();
        println!("length: {length}");

        // properties - I need to learn how this works, but this is following pika logic
        let mut flags = 0_u64;
        let mut flag_index = 0_u16;
        loop {
            let partial_flags = dec.take_u16() as u64;
            flags = flags | (partial_flags << (flag_index * 16));
            if (partial_flags & 1) == 0 {
                break;
            } else {
                flag_index += 1;
            }
        }
        let properties = if (flags & constants::properties::HEADERS) != 0 {
            println!("We got a table!");
            dec.take_table()
        } else {
            HashMap::new()
        };
        println!("Properties: {properties:?}");
        let properties = if (flags & constants::properties::DELIVERY_MODE) != 0 {
            println!("we got a delivery mode!");
            dec.take_u8()
        } else {
            0
        };
        println!("Properties: {properties:?}");

        let header = dec.take_header();
        println!("{header:?}");
        let x = dec.take_till_frame_end();
        let y = String::from_utf8(x);
        println!("{y:?}");

        let close = Close::to_frame();
        self.write(&close);
    }

    fn new() -> Self {
        let stream = TcpStream::connect("127.0.0.1:5672").unwrap();
        let version_major = 0_u8;
        let version_minor = 0_u8;
        Self {
            stream,
            version_major,
            version_minor,
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = crate::client::Client::new("127.0.0.1:5672").await;
    println!("About to call connect");
    client.connect().await?;
    client.create_queue("test_queue").await?;
    client.consume_on_queue("test_queue").await?;
    Ok(())
}
