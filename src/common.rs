use bincode::impl_borrow_decode;

use bincode::{Decode, Encode};
#[derive(Debug)]
pub enum ClassType {
    Connection,
    Channel,
    Exchange,
    Queue,
    Basic,
    Transaction,
}

impl ClassType {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let id = u16::from_be_bytes(bytes.try_into().expect("Falied"));

        match id {
            10 => Self::Connection,
            20 => Self::Channel,
            40 => Self::Exchange,
            50 => Self::Queue,
            60 => Self::Basic,
            90 => Self::Transaction,
            _ => {
                println!("ID is : {id}");
                todo!("Oh no couldn't get class type");
            }
        }
    }
}

#[derive(Debug)]
pub enum ConnectionClassMethodType {
    Start,
    StartOk,
    Secure,
    SecureOk,
    Tune,
    TuneOk,
    Open,
    OpenOk,
    Close,
    CloseOk,
}

impl ConnectionClassMethodType {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let id = u16::from_be_bytes(bytes.try_into().expect("F"));
        match id {
            10 => Self::Start,
            11 => Self::StartOk,
            20 => Self::Secure,
            21 => Self::SecureOk,
            30 => Self::Tune,
            31 => Self::TuneOk,
            40 => Self::Open,
            41 => Self::OpenOk,
            50 => Self::Close,
            51 => Self::CloseOk,
            _ => Self::Start,
        }
    }
    // TODO! Generate to bytes method
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum FrameType {
    Body,
    FatalError,
    Header,
    Heartbeat,
    Method,
}

impl Encode for FrameType {
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

impl FrameType {
    pub fn from_octet(byte: u8) -> Self {
        println!("Byte is: {byte:?}");
        match byte {
            1 => Self::Method,
            2 => Self::Header,
            3 => Self::Body,
            8 => Self::Heartbeat, // Why is this 8, the spec says it should be 4
            _ => Self::FatalError,
        }
    }

    pub fn as_octet(&self) -> u8 {
        match self {
            FrameType::Method => 1,
            FrameType::Header => 2,
            FrameType::Body => 3,
            FrameType::Heartbeat => 4,
            FrameType::FatalError => todo!(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Header {
    pub frame_type: FrameType,
    pub channel: u16,
    pub size: u32,
}

impl Encode for Header {
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
        self.channel.encode(encoder)?;
        self.size.encode(encoder)?;
        Ok(())
    }
}

impl Decode for Header {
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
            channel,
            size,
        })
    }
}

impl_borrow_decode!(Header);

impl Header {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() == 7);
        let frame_type = FrameType::from_octet(bytes[0]);
        let channel = u16::from_be_bytes(bytes[1..3].try_into().expect("failed"));
        let size = u32::from_be_bytes(bytes[3..].try_into().expect("F"));
        Self {
            frame_type,
            channel,
            size,
        }
    }
}
