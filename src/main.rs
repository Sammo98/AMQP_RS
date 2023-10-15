use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpStream;
use std::rc::Rc;

mod common;
mod communication;
mod constants;
mod method;

use method::connection::{Close, Open, ProtocolHeader, Start, StartOk, Tune, TuneOk};

use crate::communication::encode::Encoder;
use crate::communication::{decode, encode, Value};
use crate::constants::connection_method_id::START;

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
        self.stream.read(&mut buffer).unwrap();
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

        let close = Close::to_frame();
        self.write(&close);

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
