use crate::encde::*;

#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
pub struct Body {
    header: Header,
    content: RawBytes,
    frame_end: u8,
}

#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
pub struct BodyReceive {
    header: Header,
    content: RawBytes,
}

impl BodyReceive {
    pub fn inner(self) -> Vec<u8> {
        self.content.0
    }
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
