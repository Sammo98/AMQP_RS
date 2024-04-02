use bincode::config::{BigEndian, Configuration, Fixint};

pub const FRAME_END: u8 = 206; // xCE

pub const PROTOCOL_HEADER: [u8; 8] = [65, 77, 81, 80, 0, 0, 9, 1]; // "AMQP0091"
pub const CONFIG: Configuration<BigEndian, Fixint> = bincode::config::standard()
    .with_big_endian()
    .with_fixed_int_encoding();

pub const FRAME_MAX_SIZE: usize = 4096;

// pub mod properties {
//     pub const CONTENT_TYPE: u64 = 1 << 15;
//     pub const CONTENT_ENCODING: u64 = 1 << 14;
//     pub const HEADERS: u64 = 1 << 13;
//     pub const DELIVERY_MODE: u64 = 1 << 12;
//     pub const PRIORITY: u64 = 1 << 11;
//     pub const CORRELATION_ID: u64 = 1 << 10;
//     pub const REPLY_TO: u64 = 1 << 9;
//     pub const EXPIRATION: u64 = 1 << 8;
//     pub const MESSAGE_ID: u64 = 1 << 7;
//     pub const TIMESTAMP: u64 = 1 << 6;
//     pub const TYPE: u64 = 1 << 5;
//     pub const USER_ID: u64 = 1 << 4;
//     pub const APP_ID: u64 = 1 << 3;
//     pub const CLUSTER_ID: u64 = 1 << 2;
// }
