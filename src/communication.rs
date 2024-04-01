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

        pub fn print_remainder(&self) {
            let offset = self.get_current_offset();
            let current = &self.buffer[offset..];
            println!("{:?}", &current);
        }

        pub fn next_frame(&self) {
            let offset = self.get_current_offset();
            let current = &self.buffer[offset];
            assert_eq!(current, &0xCE);
            self.increment_offset_by(1);
        }

        pub fn take_till_frame_end(&self) -> Vec<u8> {
            let offset = self.get_current_offset();
            let res = self.buffer[offset..]
                .iter()
                .take_while(|&x| x != &0xCE)
                .map(|x| *x)
                .collect::<Vec<u8>>();
            res
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

        // TODO THIS IS WRONG
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

        pub fn take_u64(&self) -> u64 {
            let offset = self.get_current_offset();
            let bytes = &self.buffer[offset..offset + size::U64_SIZE];
            self.increment_offset_by(size::U64_SIZE);
            u64::from_be_bytes(bytes.try_into().expect("x"))
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
