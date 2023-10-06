use crate::common::ClassType;
use crate::common::ConnectionClassMethodType;
use crate::common::Header;
use crate::constants::CLASS_OR_METHOD_TYPE_SIZE;
use crate::constants::HEADER_SIZE;
use crate::constants::U16_SIZE;
use crate::constants::U32_SIZE;
use crate::constants::U8_SIZE;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct FrameDecoder<'a> {
    frame: &'a [u8],
    offset: RefCell<usize>,
}

impl<'a> FrameDecoder<'a> {
    pub fn new(frame: &'a [u8], offset: usize) -> Self {
        Self {
            frame,
            offset: RefCell::new(offset),
        }
    }

    fn get_current_offset(&self) -> usize {
        *self.offset.borrow()
    }

    fn increment_offset_by(&self, increment: usize) {
        self.offset.replace_with(|&mut old| old + increment);
    }

    pub fn decode_header(&mut self) -> Header {
        let offset = self.get_current_offset();
        let bytes = &self.frame[offset..offset + HEADER_SIZE];
        // We don't increment the offset, but instead move self.frame += 7
        // Essentially making self.frame the body
        self.frame = &self.frame[7..];
        Header::from_bytes(bytes)
    }

    pub fn decode_class_type(&self) -> ClassType {
        let offset = self.get_current_offset();
        let bytes = &self.frame[offset..offset + CLASS_OR_METHOD_TYPE_SIZE];
        self.increment_offset_by(CLASS_OR_METHOD_TYPE_SIZE);
        ClassType::from_bytes(bytes)
    }

    pub fn decode_method_type(&self) -> ConnectionClassMethodType {
        let offset = self.get_current_offset();
        let bytes = &self.frame[offset..offset + CLASS_OR_METHOD_TYPE_SIZE];
        self.increment_offset_by(CLASS_OR_METHOD_TYPE_SIZE);
        ConnectionClassMethodType::from_bytes(bytes)
    }

    pub fn get_u8(&self) -> u8 {
        let offset = self.get_current_offset();
        let bytes = &self.frame[offset..offset + U8_SIZE];
        self.increment_offset_by(U8_SIZE);
        u8::from_be_bytes(bytes.try_into().expect("x"))
    }

    pub fn get_u16(&self) -> u16 {
        let offset = self.get_current_offset();
        let bytes = &self.frame[offset..offset + U16_SIZE];
        self.increment_offset_by(U8_SIZE);
        u16::from_be_bytes(bytes.try_into().expect("x"))
    }

    pub fn get_u32(&self) -> u32 {
        let offset = self.get_current_offset();
        let bytes = &self.frame[offset..offset + U32_SIZE];
        self.increment_offset_by(U32_SIZE);
        u32::from_be_bytes(bytes.try_into().expect("x"))
    }

    pub fn decode_short_string(&self) -> &str {
        let length = self.get_u8() as usize;
        println!("Length: {length}");
        let offset = self.get_current_offset();
        let string_bytes = &self.frame[offset..offset + length];
        self.increment_offset_by(length);
        println!("String bytes: {string_bytes:?}");
        let string = std::str::from_utf8(string_bytes);
        string.unwrap()
    }

    pub fn decode_long_string(&self) -> &str {
        let length = self.get_u32() as usize;
        let offset = self.get_current_offset();
        let string_bytes = &self.frame[offset..offset + length];
        self.increment_offset_by(length);
        let string = std::str::from_utf8(string_bytes);
        string.unwrap()
    }

    fn get_field_type(&self) -> char {
        self.get_u8() as char
    }

    fn decode_value(&self) -> DecodedValue {
        let field_type = self.get_field_type();
        use DecodedValue::*;

        match field_type {
            // Boolean
            't' => Bool(self.get_u8() != 0),
            'S' => LongString(self.decode_long_string()),
            'F' => {
                let table = self.decode_table();
                println!("Table: {table:?}");
                Bool(false)
            }
            _ => Bool(false),
        }
    }

    pub fn decode_table(&self) -> HashMap<String, DecodedValue> {
        let table: HashMap<String, DecodedValue> = HashMap::new();
        let table_size = self.get_u32();
        let mut offset = self.get_current_offset();
        let limit = offset + table_size as usize;

        while offset < limit {
            let key = self.decode_short_string();
            let value = self.decode_value();
            offset = self.get_current_offset();
        }
        println!("\nExiting decode table");
        table
    }
}

#[derive(Debug)]
pub enum DecodedValue<'a> {
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
    LongString(&'a str),
}

// impl<'a> DecodedValue<'a> {
//     pub fn from_bytes(bytes: &[u8]) -> Self {}
// }
