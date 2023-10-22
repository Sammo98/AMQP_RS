use crate::communication::decode::Decoder;
use crate::communication::encode::Encoder;
use crate::communication::Value;
use crate::constants::{channel_method_id, class_id, frame_type};

pub struct Open {}

impl Open {
    pub fn to_frame() -> Vec<u8> {
        let encoder = Encoder::new();
        encoder.encode_value(Value::ShortString("1".into()), false);
        encoder.build_frame(
            frame_type::METHOD,
            class_id::CHANNEL,
            channel_method_id::OPEN,
            // Todo! why does this channel have to be 1?
            1,
        )
    }
}

pub struct OpenOk {}

impl OpenOk {
    pub fn from_frame(buffer: &[u8]) -> Self {
        let mut decoder = Decoder::new(buffer);
        _ = decoder.take_header();
        _ = decoder.take_class_type();
        _ = decoder.take_method_type();

        let res = decoder.take_long_string();
        println!("{res:?}");
        Self {}
    }
}
