use bincode::impl_borrow_decode;
use bincode::{Decode, Encode};
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct RawBytes(pub Vec<u8>);

impl Deref for RawBytes {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Encode for RawBytes {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        for byte in self.iter() {
            byte.encode(encoder)?;
        }
        Ok(())
    }
}

impl Decode for RawBytes {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let mut bytes = Vec::new();
        while let Ok(byte) = u8::decode(decoder) {
            bytes.push(byte);
        }
        Ok(Self(bytes))
    }
}
impl_borrow_decode!(RawBytes);
