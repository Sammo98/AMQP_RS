use crate::encde::*;
#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
struct ConnectionFrameInfo {
    header: Header,
    class_id: ClassID,
    method_id: ConnectionMethodID,
}

const GLOBAL_CHANNEL: u16 = 0;

#[derive(Debug, Clone, bincode::Encode)]
pub struct ProtocolHeader {
    a: u8,
    m: u8,
    q: u8,
    p: u8,
    zero: u8,
    major: u8,
    minor: u8,
    revision: u8,
}

impl ProtocolHeader {
    pub fn new() -> Self {
        Self {
            a: b'A',
            m: b'M',
            q: b'Q',
            p: b'P',
            zero: 0,
            major: 0,
            minor: 9,
            revision: 1,
        }
    }
}

#[derive(Debug, Clone, bincode::Decode)]
pub struct Start {
    frame_info: ConnectionFrameInfo,
    version_major: u8,
    version_minor: u8,
    server_properties: Table,
    pub mechanisms: LongString,
    pub locales: LongString,
}

#[derive(Debug, Clone, bincode::Encode)]
pub struct StartOk {
    frame_info: ConnectionFrameInfo,
    client_properties: Table,
    mechanism: ShortString,
    response: LongString,
    locale: ShortString,
}

impl StartOk {
    pub fn new(mechanism: &str, response: &str, locale: &str) -> Self {
        let capabilites: Table = Table(vec![
            ("authentication_failure_close".into(), Field::Bool(true)),
            ("basic.nack".into(), Field::Bool(true)),
            ("connection.blocked".into(), Field::Bool(true)),
            ("consumer_cancel_notify".into(), Field::Bool(true)),
            ("publisher_confirms".into(), Field::Bool(true)),
        ]);

        let client_properties: Table = Table(vec![
            (
                "product".to_owned(),
                Field::LS(LongString("Pika Python Client Library".into())),
            ),
            (
                "platform".into(),
                Field::LS(LongString("Python 3.11.6".into())),
            ),
            ("capabilities".into(), Field::T(capabilites)),
            (
                "information".into(),
                Field::LS(LongString("See http://pika.rtfd.org".into())),
            ),
            ("version".into(), Field::LS(LongString("2.0.0a0".into()))),
        ]);
        let header = Header {
            frame_type: FrameType::Method,
            channel_id: GLOBAL_CHANNEL,
            size: 0,
        };
        // Make these enums
        let class_id = ClassID::Connection;
        let method_id = ConnectionMethodID::StartOk;
        let frame_info = ConnectionFrameInfo {
            header,
            class_id,
            method_id,
        };
        Self {
            frame_info,
            client_properties,
            mechanism: mechanism.into(),
            response: response.into(),
            locale: locale.into(),
        }
    }
}

#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
pub struct Secure {
    frame_info: ConnectionFrameInfo,
    challenge: LongString,
}

#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
pub struct SecureOk {
    frame_info: ConnectionFrameInfo,
    response: LongString,
}
#[derive(Debug, Clone, bincode::Decode)]
pub struct Tune {
    frame_info: ConnectionFrameInfo,
    pub channel_max: u16,
    pub frame_max: u32,
    pub heartbeat: u16,
}

#[derive(Debug, Clone, bincode::Encode)]
pub struct TuneOk {
    frame_info: ConnectionFrameInfo,
    channel_max: u16,
    frame_max: u32,
    heartbeat: u16,
}

impl TuneOk {
    pub fn new(channel_max: u16, frame_max: u32, heartbeat: u16) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel_id: GLOBAL_CHANNEL,
            size: 0,
        };
        // Make these enums
        let class_id = ClassID::Connection;
        let method_id = ConnectionMethodID::TuneOk;
        let frame_info = ConnectionFrameInfo {
            header,
            class_id,
            method_id,
        };
        Self {
            frame_info,
            channel_max,
            frame_max,
            heartbeat,
        }
    }
}

#[derive(Debug, Clone, bincode::Encode)]
pub struct Open {
    frame_info: ConnectionFrameInfo,
    pub virtual_host: ShortString,
    reserved_1: u8,
    reserved_2: u8,
}

impl Open {
    pub fn new(virtual_host: &str) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel_id: GLOBAL_CHANNEL,
            size: 0,
        };
        // Make these enums
        let class_id = ClassID::Connection;
        let method_id = ConnectionMethodID::Open;
        let frame_info = ConnectionFrameInfo {
            header,
            class_id,
            method_id,
        };
        Self {
            frame_info,
            virtual_host: virtual_host.into(),
            reserved_1: RESERVED8,
            reserved_2: RESERVED8,
        }
    }
}

#[derive(Debug, Clone, bincode::Decode)]
pub struct OpenOk {
    frame_info: ConnectionFrameInfo,
    reserved_1: ShortString,
}

#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
pub struct Close {
    frame_info: ConnectionFrameInfo,
    reply_code: u16,
    reply_text: ShortString,
    closing_class_id: u16,
    closing_method_id: u16,
}

impl Close {
    pub fn new(
        reply_code: u16,
        reply_text: &str,
        closing_class_id: u16,
        closing_method_id: u16,
    ) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel_id: GLOBAL_CHANNEL,
            size: 0,
        };
        let class_id = ClassID::Connection;
        let method_id = ConnectionMethodID::Close;
        let frame_info = ConnectionFrameInfo {
            header,
            class_id,
            method_id,
        };
        Self {
            frame_info,
            reply_code,
            reply_text: reply_text.into(),
            closing_class_id,
            closing_method_id,
        }
    }
}

#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
pub struct CloseOk {
    frame_info: ConnectionFrameInfo,
}
