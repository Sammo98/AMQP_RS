use crate::{
    common::{FrameType, Header},
    constants::{class_id, frame_type, properties, FRAME_END},
};
use bincode::Encode;

#[derive(Debug, Clone, Encode)]
pub struct Content {
    header: Header,
    class_type: u16,
    weight: u16,
    size: u64,
    properties: u16,
    frame_end: u8,
}

impl Content {
    pub fn new(size: u64) -> Self {
        let header = Header {
            frame_type: FrameType::Header,
            channel: 1,
            size: 0,
        };
        let class_type = class_id::BASIC;
        let weight = 0;
        let properties = 0;
        Self {
            header,
            class_type,
            weight,
            size,
            properties,
            frame_end: FRAME_END,
        }
    }
}
