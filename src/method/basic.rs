use crate::communication::decode::Decoder;
use crate::communication::encode::Encoder;
use crate::communication::Value;
use crate::constants::WITHOUT_FIELD_TYPE;
use crate::constants::{basic_method_id, class_id, frame_type};

pub struct Publish {}

impl Publish {
    pub fn to_frame() -> Vec<u8> {
        let encoder = Encoder::new();
        //reserved
        encoder.encode_value(Value::ShortUInt(0), WITHOUT_FIELD_TYPE);
        // Exchange Name
        encoder.encode_value(Value::ShortString("".into()), WITHOUT_FIELD_TYPE);
        // Queue name (routing key)
        encoder.encode_value(Value::ShortString("my_queue".into()), WITHOUT_FIELD_TYPE);
        // madnaotry
        encoder.encode_value(Value::Bool(false), WITHOUT_FIELD_TYPE);

        encoder.build_frame(
            frame_type::METHOD,
            class_id::BASIC,
            basic_method_id::PUBLISH,
            1,
        )
    }
}
