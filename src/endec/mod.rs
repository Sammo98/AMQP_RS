pub mod bits;
pub mod class;
pub mod header;
pub mod long_string;
pub mod method;
pub mod raw_bytes;
pub mod short_string;
pub mod start;

pub use bits::Bits;
pub use class::ClassID;
pub use header::{FrameType, Header};
pub use long_string::LongString;
pub use method::{BasicMethodId, ChannelMethodID, ConnectionMethodID, QueueMethodId};
pub use raw_bytes::RawBytes;
pub use short_string::ShortString;
pub use start::{Field, Table};

const CONFIG: bincode::config::Configuration<bincode::config::BigEndian, bincode::config::Fixint> =
    bincode::config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();

// Header Length + 0xCE
const LENGTH_ADJUSTMENT: usize = 8;

const SIZE_RANGE: std::ops::Range<usize> = 3..7;

pub fn encode_frame<E: bincode::enc::Encode>(
    val: E,
) -> Result<Vec<u8>, bincode::error::EncodeError> {
    let mut bytes = bincode::encode_to_vec(&val, CONFIG)?;
    let frame_length = ((bytes.len() - LENGTH_ADJUSTMENT) as u32).to_be_bytes();
    bytes.splice(SIZE_RANGE, frame_length);
    Ok(bytes)
}

pub fn decode_frame<D: bincode::de::Decode>(src: &[u8]) -> Result<D, bincode::error::DecodeError> {
    let (result, _size): (D, usize) = bincode::decode_from_slice(src, CONFIG)?;
    Ok(result)
}
