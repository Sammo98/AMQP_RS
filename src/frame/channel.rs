use crate::endec::*;

#[derive(Debug, Clone, bincode::Encode)]
pub struct Open {
    header: Header,
    class_id: ClassID,
    method_id: ChannelMethodID,
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
        let class_id = ClassID::Channel;
        let method_id = ChannelMethodID::Open;
        let frame_end = FRAME_END;
        Self {
            header,
            class_id,
            method_id,
            reserved_1: ShortString("".into()),
            frame_end,
        }
    }
}

#[derive(Debug, Clone, bincode::Decode)]
pub struct OpenOk {
    header: Header,
    class_id: ClassID,
    method_id: ChannelMethodID,
    // Is this channel? Pika thinks so but looks like - to me
    pub reserved_1: u16,
    frame_end: u8,
}
