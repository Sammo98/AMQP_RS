use bincode::Encode;

use crate::{
    common::{FrameType, Header},
    constants::FRAME_END,
    endec::RawBytes,
};

#[derive(Debug, Clone, Encode)]
pub struct Body {
    header: Header,
    content: RawBytes,
    frame_end: u8,
}

impl Body {
    pub fn new(content: RawBytes) -> Self {
        let header = Header {
            frame_type: FrameType::Body,
            channel: 1,
            size: 0,
        };
        Self {
            header,
            content,
            frame_end: FRAME_END,
        }
    }
}
