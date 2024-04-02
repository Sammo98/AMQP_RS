use crate::constants::FRAME_END;
use crate::endec::{ClassID, FrameType, Header};
use bincode::{Decode, Encode};

#[derive(Debug, Clone, Encode, Decode)]
pub struct Content {
    header: Header,
    class_id: ClassID,
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
        let class_id = ClassID::Basic;
        let weight = 0;
        let properties = 0;
        Self {
            header,
            class_id,
            weight,
            size,
            properties,
            frame_end: FRAME_END,
        }
    }
}
