pub mod bits;
pub mod class;
pub mod exchange_type;
pub mod header;
pub mod long_string;
pub mod method;
pub mod properties;
pub mod raw_bytes;
pub mod short_string;
pub mod table;

pub use bits::Bits;
pub use class::ClassID;
pub use exchange_type::ExchangeType;
pub use header::{FrameType, Header};
pub use long_string::LongString;
pub use method::{
    BasicMethodID, ChannelMethodID, ConnectionMethodID, ExchangeMethodID, QueueMethodID,
};
pub use properties::Properties;
pub use raw_bytes::RawBytes;
pub use short_string::ShortString;
pub use table::{Field, Table};

const CONFIG: bincode::config::Configuration<bincode::config::BigEndian, bincode::config::Fixint> =
    bincode::config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
const HEADER_SIZE: usize = 7;
const SIZE_RANGE: std::ops::Range<usize> = 3..7;

pub const FRAME_END: u8 = 0xCE;
pub const RESERVED8: u8 = 0_u8;
pub const RESERVED16: u16 = 0_u16;

pub fn encode_frame<E: bincode::enc::Encode>(
    val: E,
) -> Result<Vec<u8>, bincode::error::EncodeError> {
    let mut bytes = bincode::encode_to_vec(&val, CONFIG)?;
    let frame_length = ((bytes.len() - HEADER_SIZE) as u32).to_be_bytes();
    bytes.splice(SIZE_RANGE, frame_length);
    bytes.push(FRAME_END);
    Ok(bytes)
}
pub fn encode_frame_static<E: bincode::enc::Encode>(
    val: E,
) -> Result<Vec<u8>, bincode::error::EncodeError> {
    let bytes = bincode::encode_to_vec(&val, CONFIG)?;
    Ok(bytes)
}

pub fn decode_frame<D: bincode::de::Decode>(src: &[u8]) -> Result<D, bincode::error::DecodeError> {
    let (result, _size): (D, usize) = bincode::decode_from_slice(src, CONFIG)?;
    Ok(result)
}
