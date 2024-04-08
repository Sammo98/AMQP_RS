use crate::encde::*;

#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
pub struct Body {
    pub header: Header,
    pub content: RawBytes,
}

impl Body {
    pub fn new(content: RawBytes) -> Self {
        let header = Header {
            frame_type: FrameType::Body,
            channel: 1,
            size: 0,
        };
        Self { header, content }
    }
}
