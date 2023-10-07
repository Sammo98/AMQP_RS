use std::io::prelude::*;
use std::net::TcpStream;

mod common;
mod communication;
mod constants;

use crate::communication::{decode, encode, Value};

use common::ClassType;
use common::ConnectionClassMethodType;
use common::FrameType;
use common::Header;
use constants::{size, PROTOCOL_HEADER};

#[derive(Debug)]
struct Client {
    stream: TcpStream,
    version_major: u8,
    version_minor: u8,
}

impl Client {
    fn connect(&mut self) {
        // self.stream.write(&PROTOCOL_HEADER).unwrap();

        // let mut buffer = [0; 1024];
        // self.stream.read(&mut buffer).unwrap();

        // let mut decoder = FrameDecoder::new(&buffer, 0);

        // // These steps happen in a specific and defined order as the decoder keeps track off the buffer offset state
        // let header = decoder.take_header();
        // let class_type = decoder.take_class_type();
        // let method_type = decoder.take_method_type();

        // // Next two bytes are version major and minor
        // self.version_major = decoder.take_u8();
        // self.version_minor = decoder.take_u8();
        // let table = decoder.decode_table();
        // let mechanisms = decoder.take_long_string();
        // let locales = decoder.take_long_string();

        let encoder = encode::Encoder::new();
        let mut hash = std::collections::HashMap::new();
        hash.insert("Child Table".to_owned(), Value::Bool(false));
        let table1 = Value::Table(hash);

        let mut hash = std::collections::HashMap::new();
        hash.insert("Parent table".to_owned(), table1);
        hash.insert(
            "Parent value".into(),
            Value::ShortString("Test outer short".into()),
        );

        encoder.encode_value(Value::Table(hash));
        let frame = encoder.buffer.clone().take();
        let decoder = decode::Decoder::new(&frame);
        let first = decoder.take_next_value();
        println!("First: {first:?}");

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
