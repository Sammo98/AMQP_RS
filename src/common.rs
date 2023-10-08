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
            _ => Self::Connection,
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

#[derive(Debug)]
pub enum FrameType {
    Body,
    FatalError,
    Header,
    Heartbeat,
    Method,
}

impl FrameType {
    pub fn from_octet(byte: u8) -> Self {
        match byte {
            1 => Self::Method,
            2 => Self::Header,
            3 => Self::Body,
            4 => Self::Heartbeat,
            _ => Self::FatalError,
        }
    }

    pub fn as_octet(&self) -> u8 {
        match self {
            FrameType::Method => 1,
            FrameType::Header => 2,
            FrameType::Body => 3,
            FrameType::Heartbeat => 4,
            FrameType::FatalError => panic!(),
        }
    }
}
#[derive(Debug)]
pub struct Header {
    pub frame_type: FrameType,
    pub channel: u16,
    pub size: u32,
}

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
