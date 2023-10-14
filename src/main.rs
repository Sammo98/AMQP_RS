use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpStream;
use std::rc::Rc;

mod common;
mod communication;
mod constants;

use crate::communication::encode::Encoder;
use crate::communication::{decode, encode, Value};

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
    fn connect(&mut self) {
        self.stream.write(&PROTOCOL_HEADER).unwrap();
        // Send TuneOK and then Open, should receive OpenOk

        let mut buffer = [0; size::FRAME_MIN_SIZE];
        self.stream.read(&mut buffer).unwrap();

        let mut decoder = decode::Decoder::new(&buffer);

        // These steps happen in a specific and defined order as the decoder keeps track off the buffer offset state
        let header = decoder.take_header();
        let class_type = decoder.take_class_type();
        let method_type = decoder.take_method_type();

        // Next two bytes are version major and minor
        self.version_major = decoder.take_u8();
        self.version_minor = decoder.take_u8();
        let table = decoder.take_table();
        let mechanisms = decoder.take_long_string();
        let locales = decoder.take_long_string();

        let mut capabilities: HashMap<String, Value> = HashMap::new();
        capabilities.insert("authentication_failure_close".into(), Value::Bool(true));
        capabilities.insert("basic.nack".into(), Value::Bool(true));
        capabilities.insert("connection.blocked".into(), Value::Bool(true));
        capabilities.insert("consumer_cancel_notify".into(), Value::Bool(true));
        capabilities.insert("publisher_confirms".into(), Value::Bool(true));

        let mut properties: HashMap<String, Value> = HashMap::new();
        properties.insert("capabilities".into(), Value::Table(capabilities));
        properties.insert(
            "product".to_owned(),
            Value::LongString("Rust AMQP Client Library".into()),
        );
        properties.insert("platform".into(), Value::LongString("Rust".into()));
        let client_properties = Value::Table(properties);
        let encoder = encode::Encoder::new();
        encoder.encode_value(client_properties, false);
        encoder.encode_value(Value::ShortString("PLAIN".into()), false);
        encoder.encode_value(Value::LongString("\0guest\0guest".into()), false);
        encoder.encode_value(Value::ShortString(locales), false);

        let frame = encoder.build_frame_from_buffer(FrameType::Method, 10, 11, 0);
        self.stream.write(&frame).unwrap();

        let mut buffer = [0; size::FRAME_MIN_SIZE];
        self.stream.read(&mut buffer).unwrap();

        // TUNE
        let mut decoder = decode::Decoder::new(&buffer);
        let header = decoder.take_header();
        let class_type = decoder.take_class_type();
        let method_type = decoder.take_method_type();
        let proposed_maximum_channels = decoder.take_u16();
        let frame_max = decoder.take_u32();
        let hb = decoder.take_u16();
        // println!(
        //     "Max Channel: {}. Frame Max: {}. Heartbeat: {}",
        //     proposed_maximum_channels, frame_max, hb
        // );

        // TUNE OK
        let encoder = encode::Encoder::new();
        encoder.encode_value(Value::ShortUInt(2047), false); // Channel Max
        encoder.encode_value(Value::LongUInt(131072), false); // Frame Max
        encoder.encode_value(Value::ShortUInt(0), false); // Heart beat
        let frame = encoder.build_frame_from_buffer(FrameType::Method, 10, 31, 0);
        self.stream.write(&frame).unwrap();

        let encoder = encode::Encoder::new();
        encoder.encode_value(Value::ShortString("/".into()), false); // vhost
        encoder.encode_value(Value::ShortString("".into()), false);
        encoder.encode_value(Value::Bool(true), false);
        let frame = encoder.build_frame_from_buffer(FrameType::Method, 10, 40, 0);
        self.stream.write(&frame).unwrap();
        println!("Sleeping");
        sleep(Duration::from_secs(2));

        let mut buffer = [0; size::FRAME_MIN_SIZE];
        println!("Reading again");
        self.stream.read(&mut buffer).unwrap();

        let mut decoder = decode::Decoder::new(&buffer);
        let header = decoder.take_header();
        let class_type = decoder.take_class_type();
        let method_type = decoder.take_method_type();
        println!("{:?}, {:?}, {:?}", header, class_type, method_type);

        // I think what I want to do is have a function which matches on class_type and method_type as a tuple and that points to similar funcionality to that of spec.py class Connection etc.
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

fn main() {
    let mut client = Client::new();
    client.connect();
}
