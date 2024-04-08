#[derive(Debug, Clone)]
pub struct RawBytes(pub Vec<u8>);

impl std::ops::Deref for RawBytes {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl bincode::Encode for RawBytes {
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

impl bincode::Decode for RawBytes {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let mut bytes = Vec::new();
        while let Ok(byte) = u8::decode(decoder) {
            bytes.push(byte);
        }
        _ = bytes.pop(); // remove 0xCE
        Ok(Self(bytes))
    }
}
bincode::impl_borrow_decode!(RawBytes);
