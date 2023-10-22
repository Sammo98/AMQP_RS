use crate::constants::{field_type, frame_type, size, FRAME_END};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Value {
    Bool(bool),
    ShortShortInt(i8),
    ShortShortUInt(u8),
    ShortInt(i16),
    ShortUInt(u16),
    LongInt(i32),
    LongUInt(u32),
    LongLongInt(i64),
    LongLongUint(u64),
    Float(f32),
    Double(f64),
    Decimal, // ?
    ShortString(Box<str>),
    LongString(Box<str>),
    Table(std::collections::HashMap<String, Value>),
}

pub mod decode {
    // General purpose module for decoding.
    // Predominantly Exposes the `Decoder` struct which "takes" values from the buffer
    // Taking values consists of decoding the next value (based on field type) and subsequently increasing
    // the offset. Such that calling self.decode_value() returns the next `Value` type.
    // Certain other methods such as take header and take class and method are exposed from the Decoder
    use super::*;
    use crate::common::{ClassType, ConnectionClassMethodType, Header};

    pub struct Decoder<'a> {
        pub buffer: &'a [u8],
        pub offset: RefCell<usize>,
    }

    impl<'a> Decoder<'a> {
        pub fn new(buffer: &'a [u8]) -> Self {
            Self {
                buffer,
                offset: RefCell::new(0),
            }
        }

        fn get_current_offset(&self) -> usize {
            *self.offset.borrow()
        }

        fn increment_offset_by(&self, increment: usize) {
            self.offset.replace_with(|&mut old| old + increment);
        }

        pub fn take_header(&mut self) -> Header {
            let offset = self.get_current_offset();
            let bytes = &self.buffer[offset..offset + size::HEADER_SIZE];
            // We don't increment the offset, but instead move self.frame += 7
            // Essentially making self.frame the body
            self.buffer = &self.buffer[7..];
            Header::from_bytes(bytes)
        }

        pub fn take_class_type(&self) -> ClassType {
            let offset = self.get_current_offset();
            let bytes = &self.buffer[offset..offset + size::CLASS_OR_METHOD_TYPE_SIZE];
            self.increment_offset_by(size::CLASS_OR_METHOD_TYPE_SIZE);
            ClassType::from_bytes(bytes)
        }

        pub fn take_method_type(&self) -> ConnectionClassMethodType {
            let offset = self.get_current_offset();
            let bytes = &self.buffer[offset..offset + size::CLASS_OR_METHOD_TYPE_SIZE];
            self.increment_offset_by(size::CLASS_OR_METHOD_TYPE_SIZE);
            ConnectionClassMethodType::from_bytes(bytes)
        }

        pub fn take_u8(&self) -> u8 {
            let offset = self.get_current_offset();
            let bytes = &self.buffer[offset..offset + size::U8_SIZE];
            self.increment_offset_by(size::U8_SIZE);
            u8::from_be_bytes(bytes.try_into().expect("x"))
        }

        pub fn take_u16(&self) -> u16 {
            let offset = self.get_current_offset();
            let bytes = &self.buffer[offset..offset + size::U16_SIZE];
            self.increment_offset_by(size::U16_SIZE);
            u16::from_be_bytes(bytes.try_into().expect("x"))
        }

        pub fn take_u32(&self) -> u32 {
            let offset = self.get_current_offset();
            let bytes = &self.buffer[offset..offset + size::U32_SIZE];
            self.increment_offset_by(size::U32_SIZE);
            u32::from_be_bytes(bytes.try_into().expect("x"))
        }

        pub fn take_short_string(&self) -> Box<str> {
            let length = self.take_u8() as usize;
            let offset = self.get_current_offset();
            let string_bytes = &self.buffer[offset..offset + length];
            self.increment_offset_by(length);
            let string = std::str::from_utf8(string_bytes).unwrap();
            string.into()
        }

        pub fn take_long_string(&self) -> Box<str> {
            let length = self.take_u32() as usize;
            let offset = self.get_current_offset();
            let string_bytes = &self.buffer[offset..offset + length];
            self.increment_offset_by(length);
            let string = std::str::from_utf8(string_bytes).unwrap();
            string.into()
        }

        pub fn take_bool(&self) -> bool {
            let offset = self.get_current_offset();
            let bytes = &self.buffer[offset..offset + 1];
            self.increment_offset_by(1);
            bytes[0] != 0
        }

        fn get_field_type(&self) -> char {
            self.take_u8() as char
        }

        pub fn take_next_value(&self) -> Value {
            use field_type::*;
            use Value::*;

            let field_type = self.get_field_type();
            match field_type {
                // Boolean
                BOOL => Bool(self.take_bool()),
                LONG_STRING => LongString(self.take_long_string()),
                SHORT_STRING => ShortString(self.take_short_string()),
                TABLE => Table(self.take_table()),
                _ => Bool(false),
            }
        }

        pub fn take_table(&self) -> HashMap<String, Value> {
            let mut table: HashMap<String, Value> = HashMap::new();
            let table_size = self.take_u32();
            let limit = self.get_current_offset() + table_size as usize;

            while self.get_current_offset() < limit {
                let key = self.take_short_string();
                let value = self.take_next_value();
                table.insert(key.into(), value);
            }
            table
        }
    }
}

pub mod encode {

    use crate::common::{ClassType, FrameType};
    use crate::constants::FRAME_END;

    use super::*;

    pub struct Encoder {
        pub buffer: RefCell<Vec<u8>>,
    }
    // General purpose module for encoding.
    // Essentially does the inverse of operations in the FrameDecoder

    impl Encoder {
        pub fn new() -> Self {
            Self {
                buffer: RefCell::new(Vec::new()),
            }
        }
        pub fn build_body_frame(self, channel: u16, body: String) -> Vec<u8> {
            let mut frame: Vec<u8> = Vec::new();
            let channel = channel.to_be_bytes();
            let size = (body.len() as u32).to_be_bytes();
            let message = body.as_bytes();

            frame.push(frame_type::BODY);
            frame.extend_from_slice(&channel);
            frame.extend_from_slice(&size); // Size of message payload.

            frame.extend_from_slice(message);

            frame.push(FRAME_END);
            frame
        }

        pub fn build_content_frame(self, frame_type: u8, class_type: u16, channel: u16) -> Vec<u8> {
            let mut frame: Vec<u8> = Vec::new();
            let channel = channel.to_be_bytes();
            let properties_flags = [0_u8; 2];
            let body_size = ("Hello World!".len() as u64).to_be_bytes();
            let frame_size = (14 as u32).to_be_bytes();

            let class_bytes = class_type.to_be_bytes();

            // These 3 are make up the frame header, for the content header frame.
            frame.push(frame_type);
            frame.extend_from_slice(&channel);
            frame.extend_from_slice(&frame_size); // This is the content header frame size, same as usual

            frame.extend_from_slice(&class_bytes); // This is 2 bytes
            frame.extend_from_slice(&[0_u8; 2]); // This is the weight field, unused and must be zero for some reason?
            frame.extend_from_slice(&body_size);
            frame.extend_from_slice(&properties_flags); // Not sure about this yet ...
            frame.push(FRAME_END);
            frame
        }

        pub fn build_frame(
            self,
            frame_type: u8,
            class_type: u16,
            method_type: u16,
            channel: u16,
        ) -> Vec<u8> {
            let mut frame: Vec<u8> = Vec::new();
            let frame_body = self.buffer.take();
            let channel = channel.to_be_bytes();
            // TODO make 4 constant
            let size = (frame_body.len() as u32 + 4).to_be_bytes();

            let class_bytes = class_type.to_be_bytes();
            let method_bytes = method_type.to_be_bytes();

            frame.push(frame_type);
            frame.extend_from_slice(&channel);
            frame.extend_from_slice(&size);
            frame.extend_from_slice(&class_bytes);
            frame.extend_from_slice(&method_bytes);
            frame.extend(frame_body);
            frame.push(FRAME_END);
            frame
        }

        fn add_bool(&self, boolean: bool, with_field_type: bool) {
            let mut frame = self.buffer.borrow_mut();
            if with_field_type {
                frame.push(field_type::BOOL as u8)
            };
            frame.push(boolean as u8);
        }
        fn add_long_string(&self, string: Box<str>, with_field_type: bool) {
            let mut frame = self.buffer.borrow_mut();
            let string_bytes = string.as_bytes();
            let string_bytes_len = (string_bytes.len() as u32).to_be_bytes();
            if with_field_type {
                frame.push(field_type::LONG_STRING as u8)
            };
            frame.extend_from_slice(&string_bytes_len);
            frame.extend_from_slice(string_bytes);
        }

        fn add_short_string(&self, string: Box<str>, with_field_type: bool) {
            let mut frame = self.buffer.borrow_mut();
            let length = (string.len() as u8).to_be_bytes();
            if with_field_type {
                frame.push(field_type::SHORT_STRING as u8)
            };
            frame.extend_from_slice(&length);
            frame.extend_from_slice(string.as_bytes());
        }

        fn add_short_uint(&self, int: u16, with_field_type: bool) {
            let mut frame = self.buffer.borrow_mut();
            if with_field_type {
                frame.push(field_type::SHORT_U_INT as u8)
            };
            let bytes = int.to_be_bytes();
            frame.extend_from_slice(&bytes);
        }

        fn add_long_uint(&self, int: u32, with_field_type: bool) {
            let mut frame = self.buffer.borrow_mut();
            if with_field_type {
                frame.push(field_type::LONG_U_INT as u8)
            };
            let bytes = int.to_be_bytes();
            frame.extend_from_slice(&bytes);
        }

        fn add_table(&self, table: HashMap<String, Value>, with_field_type: bool) {
            {
                let mut frame = self.buffer.borrow_mut();
                if with_field_type {
                    frame.push(field_type::TABLE as u8)
                };
                // Make sure we drop the mut borrow
            }
            let start_length = self.buffer.borrow().len();
            for (key, value) in table.into_iter() {
                // IMPORTANT - The table field type knows that the keys are short strings.
                // Thus when encoding we do not need to specify that the key field is a short string via 115_u8
                // The default behaviour should be that when encoding a short string as part of a table, no field type identifier is added
                // Otherwise it must be added
                self.add_short_string(key.into(), false);
                self.encode_value(value, true);
            }
            let mut frame = self.buffer.borrow_mut();
            let table_length = ((frame.len() - start_length) as u32).to_be_bytes();
            frame.splice(start_length..start_length, table_length);
        }

        pub fn encode_value(&self, value: Value, with_field_type: bool) {
            use Value::*;

            match value {
                Bool(boolean) => self.add_bool(boolean, with_field_type),
                LongString(string) => self.add_long_string(string, with_field_type),
                Table(table) => self.add_table(table, with_field_type),
                ShortString(string) => self.add_short_string(string, with_field_type),
                ShortUInt(number) => self.add_short_uint(number, with_field_type),
                LongUInt(number) => self.add_long_uint(number, with_field_type),
                ShortShortInt(_) => todo!(),
                ShortShortUInt(uint) => todo!(),
                ShortInt(_) => todo!(),
                LongInt(_) => todo!(),
                LongLongInt(_) => todo!(),
                LongLongUint(_) => todo!(),
                Float(_) => todo!(),
                Double(_) => todo!(),
                Decimal => todo!(),
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use crate::common::FrameType;

    use super::decode::Decoder;
    use super::encode::Encoder;
    use super::Value;
    const WITHOUT_FIELD_TYPE: bool = false;
    const WITH_FIELD_TYPE: bool = true;

    #[test]
    fn test_bool_translation() {
        let expected_bool = false;
        let encoder = Encoder::new();
        let bool_value = Value::Bool(expected_bool);
        encoder.encode_value(bool_value, WITH_FIELD_TYPE);

        let buffer = encoder.buffer.clone().take();
        let decoder = Decoder::new(&buffer);
        let decoded_value = decoder.take_next_value();
        if let Value::Bool(b) = decoded_value {
            assert_eq!(b, expected_bool);
        } else {
            panic!("Expected Bool Value type");
        }
    }

    #[test]
    fn test_short_string_translation() {
        let expected_string: Box<str> = "TEST".into();
        let encoder = Encoder::new();
        let short_string_value = Value::ShortString(expected_string.clone());
        encoder.encode_value(short_string_value, WITH_FIELD_TYPE);

        let buffer = encoder.buffer.clone().take();
        let decoder = Decoder::new(&buffer);
        let decoded_value = decoder.take_next_value();
        if let Value::ShortString(s) = decoded_value {
            assert_eq!(s, expected_string);
        } else {
            panic!("Expected ShortString Value type.");
        }
    }

    #[test]
    fn test_long_string_translation() {
        let expected_string: Box<str> = "TEST".into();
        let encoder = Encoder::new();
        let short_string_value = Value::LongString(expected_string.clone());
        encoder.encode_value(short_string_value, WITH_FIELD_TYPE);

        let buffer = encoder.buffer.clone().take();
        let decoder = Decoder::new(&buffer);
        let decoded_value = decoder.take_next_value();
        if let Value::LongString(s) = decoded_value {
            assert_eq!(s, expected_string);
        } else {
            panic!("Expected ShortString Value type.");
        }
    }

    #[test]
    fn test_single_table_translation() {
        let value1_inner = true;
        let value2_inner: Box<str> = "TEST".into();
        let value1 = Value::Bool(value1_inner.clone());
        let value2 = Value::ShortString(value2_inner.clone());
        let mut map = HashMap::new();
        map.insert("value1".to_owned(), value1);
        map.insert("value2".to_owned(), value2);
        let table_value = Value::Table(map);
        let encoder = Encoder::new();
        encoder.encode_value(table_value, WITH_FIELD_TYPE);

        let buffer = encoder.buffer.clone().take();
        let decoder = Decoder::new(&buffer);
        let decoded_value = decoder.take_next_value();
        if let Value::Table(map) = decoded_value {
            let val1 = map.get("value1");
            assert!(val1.is_some());
            let val2 = map.get("value2");
            assert!(val2.is_some());
        } else {
            panic!("Expected Table Value type")
        }
    }

    #[test]
    fn test_start_ok() {
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

        let encoder = Encoder::new();
        encoder.encode_value(Value::Table(properties), false);
        encoder.encode_value(Value::ShortString("Mechanisms".into()), false);
        encoder.encode_value(Value::LongString("Response".into()), false);
        encoder.encode_value(Value::ShortString("Locales".into()), false);

        let frame = encoder.build_frame(1, 10, 11, 0);

        let mut decoder = Decoder::new(&frame);
        let header = decoder.take_header();
        let class = decoder.take_class_type();
        let method = decoder.take_method_type();

        let properties = decoder.take_table();
        println!("Properties: {properties:?}");

        let mechanisms = decoder.take_short_string();
        println!("Mechanisms: {mechanisms:?}");
        let response = decoder.take_long_string();
        println!("Response: {response:?}");
        let locales = decoder.take_short_string();
        println!("locales: {locales:?}");
        let offset = decoder.offset.take();
        let header_size = header.size;
        println!(
            "Header size + 8: {}. Actual size {}",
            header_size + 7,
            frame.len()
        );
        let remainder = &decoder.buffer[offset..];
        println!("Remainder: {remainder:?}");
    }
}
