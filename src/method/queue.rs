use std::collections::HashMap;

use crate::communication::encode::Encoder;
use crate::communication::Value;
use crate::constants::WITHOUT_FIELD_TYPE;
use crate::constants::{class_id, frame_type, queue_method_id};

pub struct Declare {}

impl Declare {
    pub fn to_frame(
        queue_name: &str,
        passive: bool,
        durable: bool,
        exclusive: bool,
        auto_delete: bool,
        no_wait: bool,
    ) -> Vec<u8> {
        let encoder = Encoder::new();
        // Reserved
        encoder.encode_value(Value::ShortUInt(0), WITHOUT_FIELD_TYPE);
        // Q name
        encoder.encode_value(Value::ShortString(queue_name.into()), WITHOUT_FIELD_TYPE);

        let mut bit_buffer: u8 = 0b0000_0000;
        if passive {
            bit_buffer |= 1 << 0;
        }
        if durable {
            bit_buffer |= 1 << 1;
        }
        if exclusive {
            bit_buffer |= 1 << 2;
        }
        if auto_delete {
            bit_buffer |= 1 << 3;
        }
        if no_wait {
            bit_buffer |= 1 << 4;
        }

        encoder.encode_value(Value::ShortShortUInt(bit_buffer), WITHOUT_FIELD_TYPE);

        // Arguments, TODO what could this be? Impl specific apparently
        encoder.encode_value(Value::Table(HashMap::new()), WITHOUT_FIELD_TYPE);

        encoder.build_frame(
            frame_type::METHOD,
            class_id::QUEUE,
            queue_method_id::DECLARE,
            1,
        )
    }
}
