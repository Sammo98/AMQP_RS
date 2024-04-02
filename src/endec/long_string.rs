use bincode::impl_borrow_decode;
use bincode::{Decode, Encode};
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct LongString(pub String);

impl Deref for LongString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Encode for LongString {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        let LongString(inner) = self;
        let length = (inner.len() as u32).to_be_bytes();
        for x in length {
            x.encode(encoder)?;
        }
        // length.encode(encoder)?;
        for c in inner.chars() {
            (c as u8).encode(encoder)?;
        }
        Ok(())
    }
}

impl Decode for LongString {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let length = u32::decode(decoder)?;
        let mut string_bytes = vec![];
        for _ in 0..length {
            let byte = u8::decode(decoder)?;
            string_bytes.push(byte);
        }
        let decoded_string = String::from_utf8(string_bytes).expect("Fatal Decode Error");
        Ok(Self(decoded_string))
    }
}

impl_borrow_decode!(LongString);
