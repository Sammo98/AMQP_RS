use crate::constants::{field_type, size};
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
        offset: RefCell<usize>,
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
                TABLE => Table(self.decode_table()),
                _ => Bool(false),
            }
        }

        fn decode_table(&self) -> HashMap<String, Value> {
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

        fn build_frame<'b>(self) -> &'b [u8] {
            todo!()
        }

        fn add_bool(&self, boolean: bool) {
            let mut frame = self.buffer.borrow_mut();
            frame.push(field_type::BOOL as u8);
            frame.push(boolean as u8);
        }
        fn add_long_string(&self, string: Box<str>) {
            let mut frame = self.buffer.borrow_mut();
            let string_size = (string.len() as u32).to_be_bytes();
            frame.push(field_type::LONG_STRING as u8);
            frame.extend_from_slice(&string_size);
            frame.extend_from_slice(string.as_bytes());
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

        fn add_table(&self, table: HashMap<String, Value>) {
            {
                let mut frame = self.buffer.borrow_mut();
                frame.push(field_type::TABLE as u8);
                // Make sure we drop the mut borrow
            }
            let start_length = self.buffer.borrow().len();
            for (key, value) in table.into_iter() {
                // IMPORTANT - The table field type knows that the keys are short strings.
                // Thus when encoding we do not need to specify that the key field is a short string via 115_u8
                // The default behaviour should be that when encoding a short string as part of a table, no field type identifier is added
                // Otherwise it must be added
                self.add_short_string(key.into(), false);
                self.encode_value(value);
            }
            let mut frame = self.buffer.borrow_mut();
            let table_length = ((frame.len() - start_length) as u32).to_be_bytes();
            frame.splice(start_length..start_length, table_length);
        }

        pub fn encode_value(&self, value: Value) {
            use Value::*;

            match value {
                Bool(boolean) => self.add_bool(boolean),
                LongString(string) => self.add_long_string(string),
                Table(table) => self.add_table(table),
                ShortString(string) => self.add_short_string(string, true),
                ShortShortInt(_) => todo!(),
                ShortShortUInt(_) => todo!(),
                ShortInt(_) => todo!(),
                ShortUInt(_) => todo!(),
                LongInt(_) => todo!(),
                LongUInt(_) => todo!(),
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

    use super::decode::Decoder;
    use super::encode::Encoder;
    use super::Value;

    #[test]
    fn test_bool_translation() {
        let expected_bool = false;
        let encoder = Encoder::new();
        let bool_value = Value::Bool(expected_bool);
        encoder.encode_value(bool_value);

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
        encoder.encode_value(short_string_value);

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
        encoder.encode_value(short_string_value);

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
        encoder.encode_value(table_value);

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
}
