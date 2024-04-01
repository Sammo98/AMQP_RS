use crate::common::FrameType;
use crate::common::Header;
use crate::constants::class_id;
use crate::constants::connection_method_id;
use crate::endec;
use crate::endec::{LongString, ShortString};
use bincode::{Decode, Encode};

#[derive(Debug, Clone, Encode, Decode)]
pub struct Start {
    header: Header,
    class_type: u16,
    method_type: u16,
    version_major: u8,
    version_minor: u8,
    server_properties: endec::Table,
    pub mechanisms: endec::LongString,
    pub locales: endec::LongString,
    frame_end: u8,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct StartOk {
    header: Header,
    class_type: u16,
    method_type: u16,
    client_properties: endec::Table,
    mechanism: endec::ShortString,
    response: endec::LongString,
    locale: endec::ShortString,
    frame_end: u8,
}

impl StartOk {
    pub fn new(mechanism: String, response: String, locale: String) -> Self {
        let capabilites: endec::Table = endec::Table(vec![
            (
                "authentication_failure_close".into(),
                endec::Field::Bool(true),
            ),
            ("basic.nack".into(), endec::Field::Bool(true)),
            ("connection.blocked".into(), endec::Field::Bool(true)),
            ("consumer_cancel_notify".into(), endec::Field::Bool(true)),
            ("publisher_confirms".into(), endec::Field::Bool(true)),
        ]);

        let client_properties: endec::Table = endec::Table(vec![
            ("capabilities".into(), endec::Field::T(capabilites)),
            (
                "product".to_owned(),
                endec::Field::LS(LongString("Rust AMQP Client Library".into())),
            ),
            (
                "platform".into(),
                endec::Field::LS(LongString("Rust".into())),
            ),
        ]);
        let header = Header {
            frame_type: FrameType::Method,
            channel: 0,
            size: 0,
        };
        // Make these enums
        let class_type = class_id::CONNECTION;
        let method_type = connection_method_id::STARTOK;
        let frame_end = 0xCE;
        Self {
            header,
            class_type,
            method_type,
            client_properties,
            mechanism: ShortString(mechanism),
            response: LongString(response),
            locale: ShortString(locale),
            frame_end,
        }
    }
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct Tune {
    header: Header,
    class_type: u16,
    method_type: u16,
    pub channel_max: u16,
    pub frame_max: u32,
    pub heartbeat: u16,
    frame_end: u8,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct TuneOk {
    header: Header,
    class_type: u16,
    method_type: u16,
    channel_max: u16,
    frame_max: u32,
    heartbeat: u16,
    frame_end: u8,
}

impl TuneOk {
    pub fn new(channel_max: u16, frame_max: u32, heartbeat: u16) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel: 0,
            size: 0,
        };
        // Make these enums
        let class_type = class_id::CONNECTION;
        let method_type = connection_method_id::TUNEOK;
        let frame_end = 0xCE;
        Self {
            header,
            class_type,
            method_type,
            channel_max,
            frame_max,
            heartbeat,
            frame_end,
        }
    }
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct Open {
    header: Header,
    class_type: u16,
    method_type: u16,
    pub virtual_host: ShortString,
    pub reserved_1: ShortString,
    pub reserved_2: bool,
    frame_end: u8,
}

impl Open {
    pub fn new(virtual_host: String, reserved_1: String, reserved_2: bool) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel: 0,
            size: 0,
        };
        let virtual_host = ShortString(virtual_host);
        let reserved_1 = ShortString(reserved_1);
        // Make these enums
        let class_type = class_id::CONNECTION;
        let method_type = connection_method_id::OPEN;
        let frame_end = 0xCE;
        Self {
            header,
            class_type,
            method_type,
            virtual_host,
            reserved_1,
            reserved_2,
            frame_end,
        }
    }
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct Close {
    header: Header,
    class_type: u16,
    method_type: u16,
    reply_code: u16,
    reply_text: ShortString,
    closing_class_id: u16,
    closing_method_id: u16,
    frame_end: u8,
}

impl Close {
    pub fn new(
        reply_code: u16,
        reply_text: ShortString,
        closing_class_id: u16,
        closing_method_id: u16,
    ) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel: 0,
            size: 0,
        };
        let class_type = class_id::CONNECTION;
        let method_type = connection_method_id::CLOSE;
        let frame_end = 0xCE;
        Self {
            header,
            class_type,
            method_type,
            reply_code,
            reply_text,
            closing_class_id,
            closing_method_id,
            frame_end,
        }
    }
}
