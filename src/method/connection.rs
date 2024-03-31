use crate::common::Header;
use crate::communication::decode::Decoder;
use crate::communication::encode::Encoder;
use crate::communication::Value;
use crate::constants::PROTOCOL_HEADER;
use crate::constants::WITHOUT_FIELD_TYPE;
use crate::constants::{class_id, connection_method_id, frame_type};
use crate::endec;
use bincode::{Decode, Encode};
use std::collections::HashMap;
pub struct ProtocolHeader {
    // Note - this isn't officially a connection method, but has be included here
    // as it kicks off the connection process
}

impl ProtocolHeader {
    pub fn to_frame() -> [u8; 8] {
        PROTOCOL_HEADER
    }
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct Start {
    header: Header,
    class_type: u16,
    method_type: u16,
    version_major: u8,
    version_minor: u8,
    server_properties: endec::Table,
    pub mechanisms: endec::LongString,
    pub locales: endec::LongString,
    frame_end: u8,
}

pub struct StartOk {
    _client_properties: HashMap<String, Value>,
    _mechanism: Box<str>,
    _response: Box<str>,
    _locale: Box<str>,
}

impl StartOk {
    pub fn to_frame(mechanism: Box<str>, response: Box<str>, locale: Box<str>) -> Vec<u8> {
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
        let encoder = Encoder::new();
        encoder.encode_value(client_properties, WITHOUT_FIELD_TYPE);
        encoder.encode_value(Value::ShortString(mechanism), WITHOUT_FIELD_TYPE); // "PLAIN"
        encoder.encode_value(Value::LongString(response), WITHOUT_FIELD_TYPE); // "\0guest\0guest"
        encoder.encode_value(Value::ShortString(locale), WITHOUT_FIELD_TYPE);

        encoder.build_frame(
            frame_type::METHOD,
            class_id::CONNECTION,
            connection_method_id::STARTOK,
            0,
        )
    }
}

pub struct Tune {
    pub channel_max: u16,
    pub frame_max: u32,
    pub heartbeat: u16,
}

impl Tune {
    pub fn from_frame(buffer: &[u8]) -> Self {
        let mut decoder = Decoder::new(buffer);

        let header = decoder.take_header();
        println!("{header:?}");
        _ = decoder.take_class_type();
        _ = decoder.take_method_type();

        let channel_max = decoder.take_u16();
        let frame_max = decoder.take_u32();
        let heartbeat = decoder.take_u16();

        Self {
            channel_max,
            frame_max,
            heartbeat,
        }
    }
}

pub struct TuneOk {}

impl TuneOk {
    pub fn to_frame(channel_max: u16, frame_max: u32, heartbeat: u16) -> Vec<u8> {
        let encoder = Encoder::new();
        encoder.encode_value(Value::ShortUInt(channel_max), WITHOUT_FIELD_TYPE);
        encoder.encode_value(Value::LongUInt(frame_max), WITHOUT_FIELD_TYPE);
        encoder.encode_value(Value::ShortUInt(heartbeat), WITHOUT_FIELD_TYPE);
        encoder.build_frame(
            frame_type::METHOD,
            class_id::CONNECTION,
            connection_method_id::TUNEOK,
            0,
        )
    }
}

pub struct Open {}

impl Open {
    pub fn to_frame(virtual_host: Box<str>, reserved_1: Box<str>, reserved_2: bool) -> Vec<u8> {
        let encoder = Encoder::new();
        encoder.encode_value(Value::ShortString(virtual_host), WITHOUT_FIELD_TYPE); // vhost
        encoder.encode_value(Value::ShortString(reserved_1), WITHOUT_FIELD_TYPE);
        encoder.encode_value(Value::Bool(reserved_2), WITHOUT_FIELD_TYPE);
        encoder.build_frame(
            frame_type::METHOD,
            class_id::CONNECTION,
            connection_method_id::OPEN,
            0,
        )
    }
}

pub struct Close {}

impl Close {
    pub fn to_frame() -> Vec<u8> {
        let encoder = Encoder::new();
        encoder.encode_value(Value::ShortUInt(200), WITHOUT_FIELD_TYPE);
        encoder.encode_value(
            Value::ShortString("Normal Shutdown".into()),
            WITHOUT_FIELD_TYPE,
        );
        encoder.encode_value(Value::ShortUInt(0), WITHOUT_FIELD_TYPE);
        encoder.encode_value(Value::ShortUInt(0), WITHOUT_FIELD_TYPE);
        encoder.build_frame(
            frame_type::METHOD,
            class_id::CONNECTION,
            connection_method_id::CLOSE,
            0,
        )
    }
}
