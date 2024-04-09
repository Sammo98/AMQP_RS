#[derive(Debug, Clone)]
pub struct ShortString(pub String);

impl From<&str> for ShortString {
    fn from(value: &str) -> Self {
        ShortString(value.into())
    }
}
impl std::ops::Deref for ShortString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl bincode::Encode for ShortString {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        let ShortString(inner) = self;
        let length = inner.len() as u8;
        length.encode(encoder)?;
        for c in inner.chars() {
            (c as u8).encode(encoder)?;
        }
        Ok(())
    }
}

impl bincode::Decode for ShortString {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        // Take exact, u8 then into vec and so on
        let length = u8::decode(decoder)?;
        let mut string_bytes = vec![];
        for _ in 0..length {
            let byte = u8::decode(decoder)?;
            string_bytes.push(byte);
        }
        let decoded_string = String::from_utf8(string_bytes).expect("Fatal Decode Error");
        Ok(Self(decoded_string))
    }
}
bincode::impl_borrow_decode!(ShortString);
