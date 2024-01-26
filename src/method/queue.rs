use crate::communication::decode::Decoder;
use crate::communication::encode::Encoder;
use crate::communication::Value;
use crate::constants::WITHOUT_FIELD_TYPE;
use crate::constants::{class_id, frame_type, queue_method_id};

pub struct Declare {}

impl Declare {
    pub fn to_frame() -> Vec<u8> {
        let encoder = Encoder::new();
        // Reserved
        encoder.encode_value(Value::ShortUInt(0), WITHOUT_FIELD_TYPE);
        // Q name
        encoder.encode_value(Value::ShortString("my_queue".into()), WITHOUT_FIELD_TYPE);
        // Passive
        encoder.encode_value(Value::Bool(false), WITHOUT_FIELD_TYPE);
        // Durable
        encoder.encode_value(Value::Bool(false), WITHOUT_FIELD_TYPE);
        //Exclusive
        encoder.encode_value(Value::Bool(false), WITHOUT_FIELD_TYPE);
        // Auto-delete
        encoder.encode_value(Value::Bool(false), WITHOUT_FIELD_TYPE);
        // no-wait
        encoder.encode_value(Value::Bool(false), WITHOUT_FIELD_TYPE);
        encoder.build_frame(
            frame_type::METHOD,
            class_id::QUEUE,
            queue_method_id::DECLARE,
            1,
        )
    }
}
