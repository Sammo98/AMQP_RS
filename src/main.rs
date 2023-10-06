use std::io::prelude::*;
use std::net::TcpStream;

mod common;
mod constants;
mod decode;

use common::ClassType;
use common::ConnectionClassMethodType;
use common::FrameType;
use common::Header;
use constants::HEADER_SIZE;
use constants::PROTOCOL_HEADER;
use decode::FrameDecoder;

#[derive(Debug)]
struct Client {
    stream: TcpStream,
    version_major: u8,
    version_minor: u8,
}

impl Client {
    fn connect(&mut self) {
        self.stream.write(&PROTOCOL_HEADER).unwrap();

        let mut buffer = [0; 1024];
        self.stream.read(&mut buffer).unwrap();

        let mut decoder = FrameDecoder::new(&buffer, 0);

        // These steps happen in a specific and defined order as the decoder keeps track off the buffer offset state
        let header = decoder.decode_header();
        let class_type = decoder.decode_class_type();
        let method_type = decoder.decode_method_type();

        // Next two bytes are version major and minor
        self.version_major = decoder.get_u8();
        self.version_minor = decoder.get_u8();
        let table = decoder.decode_table();

        // decode_table(&frame_data[offset..]);
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
