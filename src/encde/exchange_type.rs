use super::ShortString;

#[derive(Debug, Clone)]
pub enum ExchangeType {
    Direct,
    Fanout,
    Headers,
    Topic,
}

impl bincode::Encode for ExchangeType {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        let val = match self {
            ExchangeType::Direct => ShortString("direct".into()),
            ExchangeType::Fanout => ShortString("fanout".into()),
            ExchangeType::Headers => ShortString("headers".into()),
            ExchangeType::Topic => ShortString("topic".into()),
        };
        val.encode(encoder)?;
        Ok(())
    }
}
