use crate::encde::*;

#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
pub struct Body {
    pub header: Header,
    pub content: RawBytes,
}

impl Body {
    pub fn new(channel_id: u16, content: RawBytes) -> Self {
        let header = Header {
            frame_type: FrameType::Body,
            channel_id,
            size: 0,
        };
        Self { header, content }
    }
}
