#[derive(Debug, Clone)]
pub struct Bits(pub Vec<u8>);

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
