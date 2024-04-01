use crate::common::{FrameType, Header};
use crate::constants::{channel_method_id, class_id};
use crate::endec::ShortString;
use bincode::{Decode, Encode};

#[derive(Debug, Clone, Encode)]
pub struct Open {
    header: Header,
    class_type: u16,
    method_type: u16,
    reserved_1: ShortString,
    frame_end: u8,
}

impl Open {
    pub fn new() -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel: 1,
            size: 0,
        };
        let class_type = class_id::CHANNEL;
        let method_type = channel_method_id::OPEN;
        let frame_end = 0xCE;
        Self {
            header,
            class_type,
            method_type,
            reserved_1: ShortString("1".into()),
            frame_end,
        }
    }
}

#[derive(Debug, Clone, Decode)]
pub struct OpenOk {
    header: Header,
    class_type: u16,
    method_type: u16,
    // Is this channel? Pika thinks so but looks like - to me
    pub reserved_1: u16,
    frame_end: u8,
}
