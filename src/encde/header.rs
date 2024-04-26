#[derive(Debug, Eq, PartialEq, Clone)]
pub enum FrameType {
    Body,
    FatalError,
    Header,
    Heartbeat,
    Method,
}

impl bincode::Encode for FrameType {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        match self {
            FrameType::Body => 3.encode(encoder)?,
            FrameType::FatalError => todo!(),
            FrameType::Header => 2.encode(encoder)?,
            FrameType::Heartbeat => 8.encode(encoder)?,
            FrameType::Method => 1.encode(encoder)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Header {
    pub frame_type: FrameType,
    pub channel_id: u16,
    pub size: u32,
}

impl bincode::Encode for Header {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        match self.frame_type {
            FrameType::Method => 0x01_u8.encode(encoder)?,
            FrameType::Header => 0x02_u8.encode(encoder)?,
            FrameType::Body => 0x03_u8.encode(encoder)?,
            FrameType::Heartbeat => 0x08_u8.encode(encoder)?,
            FrameType::FatalError => todo!(""),
        }
        self.channel_id.encode(encoder)?;
        self.size.encode(encoder)?;
        Ok(())
    }
}

impl bincode::Decode for Header {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let frame_type = match u8::decode(decoder)? {
            0x01 => FrameType::Method,
            0x02 => FrameType::Header,
            0x03 => FrameType::Body,
            0x08 => FrameType::Heartbeat,
            _ => FrameType::FatalError,
        };
        let channel = u16::decode(decoder)?;
        let size = u32::decode(decoder)?;
        Ok(Self {
            frame_type,
            channel_id: channel,
            size,
        })
    }
}

bincode::impl_borrow_decode!(Header);
