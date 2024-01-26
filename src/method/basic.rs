use std::collections::HashMap;

use crate::communication::decode::Decoder;
use crate::communication::encode::Encoder;
use crate::communication::Value;
use crate::constants::WITHOUT_FIELD_TYPE;
use crate::constants::{basic_method_id, class_id, frame_type};

const QUEUE: &'static str = "my_queue";
pub struct Publish {}

impl Publish {
    pub fn to_frame() -> Vec<u8> {
        let encoder = Encoder::new();
        //reserved
        encoder.encode_value(Value::ShortUInt(0), WITHOUT_FIELD_TYPE);
        // Exchange Name
        encoder.encode_value(Value::ShortString("".into()), WITHOUT_FIELD_TYPE);
        // Queue name (routing key)
        encoder.encode_value(Value::ShortString(QUEUE.into()), WITHOUT_FIELD_TYPE);
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

pub struct Consume {}

impl Consume {
    pub fn to_frame() -> Vec<u8> {
        let encoder = Encoder::new();
        //reserved
        encoder.encode_value(Value::ShortUInt(0), WITHOUT_FIELD_TYPE);
        // Queue name (routing key)
        encoder.encode_value(Value::ShortString(QUEUE.into()), WITHOUT_FIELD_TYPE);
        // Consumer tag
        encoder.encode_value(
            Value::ShortString("CONSUMER_TAG".into()),
            WITHOUT_FIELD_TYPE,
        );
        // no local
        // Only accepts one bit, so this one bool will apply to the next 3 fields, need to work out how to send!
        // Presumably one u8 that we bit shift
        encoder.encode_value(Value::Bool(false), WITHOUT_FIELD_TYPE);
        // no ack
        // encoder.encode_value(Value::Bool(false), WITHOUT_FIELD_TYPE);
        // exclusive
        // encoder.encode_value(Value::Bool(false), WITHOUT_FIELD_TYPE);
        // no-wait
        // encoder.encode_value(Value::Bool(false), WITHOUT_FIELD_TYPE);
        encoder.encode_value(Value::Table(HashMap::new()), WITHOUT_FIELD_TYPE);

        encoder.build_frame(
            frame_type::METHOD,
            class_id::BASIC,
            basic_method_id::CONSUME,
            1,
        )
    }
}
