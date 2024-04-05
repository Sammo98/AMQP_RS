#[derive(Debug, Clone)]
pub enum ClassID {
    Connection,
    Channel,
    Exchange,
    Queue,
    Basic,
    Transaction,
}

impl bincode::Encode for ClassID {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        match self {
            ClassID::Connection => 10_u16.encode(encoder)?,
            ClassID::Channel => 20_u16.encode(encoder)?,
            ClassID::Exchange => 40_u16.encode(encoder)?,
            ClassID::Queue => 50_u16.encode(encoder)?,
            ClassID::Basic => 60_u16.encode(encoder)?,
            ClassID::Transaction => 90_u16.encode(encoder)?,
        }
        Ok(())
    }
}

impl bincode::Decode for ClassID {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        Ok(match u16::decode(decoder)? {
            10_u16 => ClassID::Connection,
            20_u16 => ClassID::Channel,
            40_u16 => ClassID::Exchange,
            50_u16 => ClassID::Queue,
            60_u16 => ClassID::Basic,
            90_u16 => ClassID::Transaction,
            _ => todo!(),
        })
    }
}
bincode::impl_borrow_decode!(ClassID);
