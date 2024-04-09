#[derive(Debug, Clone, Default)]
pub struct Bits(pub Vec<u8>);

impl From<(bool,)> for Bits {
    fn from(value: (bool,)) -> Self {
        Self(vec![value.0 as u8])
    }
}
impl From<(bool, bool)> for Bits {
    fn from(value: (bool, bool)) -> Self {
        Self(vec![value.0 as u8, value.1 as u8])
    }
}
impl From<(bool, bool, bool)> for Bits {
    fn from(value: (bool, bool, bool)) -> Self {
        Self(vec![value.0 as u8, value.1 as u8, value.2 as u8])
    }
}
impl From<(bool, bool, bool, bool)> for Bits {
    fn from(value: (bool, bool, bool, bool)) -> Self {
        Self(vec![
            value.0 as u8,
            value.1 as u8,
            value.2 as u8,
            value.3 as u8,
        ])
    }
}
impl From<(bool, bool, bool, bool, bool)> for Bits {
    fn from(value: (bool, bool, bool, bool, bool)) -> Self {
        Self(vec![
            value.0 as u8,
            value.1 as u8,
            value.2 as u8,
            value.3 as u8,
            value.4 as u8,
        ])
    }
}

impl std::ops::Deref for Bits {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl bincode::Encode for Bits {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        let mut bit_buffer: u8 = 0b0000_0000;
        for (index, flag) in self.iter().enumerate() {
            if flag == &1 {
                bit_buffer |= 1 << index
            }
        }
        bit_buffer.encode(encoder)?;
        Ok(())
    }
}

impl bincode::Decode for Bits {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let flags = u8::decode(decoder)?;
        let mut bits = Vec::new();
        for i in 0_u8..8 {
            bits.push(flags & 1 << i);
        }
        Ok(Self(bits))
    }
}

bincode::impl_borrow_decode!(Bits);
