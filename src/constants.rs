use bincode::config::{BigEndian, Configuration, Fixint};

pub const FRAME_END: u8 = 206; // xCE

pub const PROTOCOL_HEADER: [u8; 8] = [65, 77, 81, 80, 0, 0, 9, 1]; // "AMQP0091"
pub const CONFIG: Configuration<BigEndian, Fixint> = bincode::config::standard()
    .with_big_endian()
    .with_fixed_int_encoding();

pub mod size {
    pub const FRAME_MAX_SIZE: usize = 4096;
}

pub mod field_type {
    pub const BOOL: char = 't';
    pub const LONG_STRING: char = 'S';
    pub const SHORT_STRING: char = 's';
    pub const TABLE: char = 'F';
    pub const SHORT_U_INT: char = 'u';
    pub const LONG_U_INT: char = 'i';
}

pub mod class_id {
    pub const CONNECTION: u16 = 10;
    pub const CHANNEL: u16 = 20;
    pub const EXCHANGE: u16 = 40;
    pub const QUEUE: u16 = 50;
    pub const BASIC: u16 = 60;
    pub const TX: u16 = 90;
}

pub mod connection_method_id {
    pub const START: u16 = 10;
    pub const STARTOK: u16 = 11;
    pub const SECURE: u16 = 20;
    pub const SECUREOK: u16 = 21;
    pub const TUNE: u16 = 30;
    pub const TUNEOK: u16 = 31;
    pub const OPEN: u16 = 40;
    pub const OPENOK: u16 = 41;
    pub const CLOSE: u16 = 50;
    pub const CLOSEOK: u16 = 51;
}

pub mod channel_method_id {
    pub const OPEN: u16 = 10;
    pub const OPENOK: u16 = 11;
    pub const FLOW: u16 = 20;
    pub const FLOWOK: u16 = 21;
    pub const CLOSE: u16 = 40;
    pub const CLOSEOK: u16 = 41;
}

pub mod queue_method_id {
    pub const DECLARE: u16 = 10;
    pub const DECLAREOK: u16 = 11;
    pub const BIND: u16 = 20;
    pub const BINDOK: u16 = 21;
    pub const PURGE: u16 = 30;
    pub const PURGEOK: u16 = 31;
    pub const DELETE: u16 = 40;
    pub const DELETEOK: u16 = 41;
    pub const UNBIND: u16 = 50;
    pub const UNBINDOK: u16 = 51;
}

pub mod basic_method_id {
    pub const QOS: u16 = 10;
    pub const QOSOK: u16 = 11;
    pub const CONSUME: u16 = 20;
    pub const CONSUMEOK: u16 = 21;
    pub const CANCEL: u16 = 30;
    pub const CANCELOK: u16 = 31;
    pub const PUBLISH: u16 = 40;
    pub const RECOVER: u16 = 110;
    pub const RECOVEROK: u16 = 111;
}

pub mod frame_type {
    pub const METHOD: u8 = 1;
    pub const HEADER: u8 = 2;
    pub const BODY: u8 = 3;
    pub const HEARTBEAT: u8 = 4;
}

pub mod properties {
    pub const CONTENT_TYPE: u64 = 1 << 15;
    pub const CONTENT_ENCODING: u64 = 1 << 14;
    pub const HEADERS: u64 = 1 << 13;
    pub const DELIVERY_MODE: u64 = 1 << 12;
    pub const PRIORITY: u64 = 1 << 11;
    pub const CORRELATION_ID: u64 = 1 << 10;
    pub const REPLY_TO: u64 = 1 << 9;
    pub const EXPIRATION: u64 = 1 << 8;
    pub const MESSAGE_ID: u64 = 1 << 7;
    pub const TIMESTAMP: u64 = 1 << 6;
    pub const TYPE: u64 = 1 << 5;
    pub const USER_ID: u64 = 1 << 4;
    pub const APP_ID: u64 = 1 << 3;
    pub const CLUSTER_ID: u64 = 1 << 2;
}
