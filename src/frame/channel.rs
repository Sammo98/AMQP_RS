use crate::encde::*;

#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
struct ChannelFrameInfo {
    header: Header,
    class_id: ClassID,
    method_id: ChannelMethodID,
}

#[derive(Debug, Clone, bincode::Encode)]
pub struct Open {
    frame_info: ChannelFrameInfo,
    reserved_1: ShortString,
}

impl Open {
    pub fn new(channel_id: u16) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel_id,
            size: 0,
        };
        let class_id = ClassID::Channel;
        let method_id = ChannelMethodID::Open;
        let frame_info = ChannelFrameInfo {
            header,
            class_id,
            method_id,
        };
        Self {
            frame_info,
            reserved_1: ShortString("".into()),
        }
    }
}

#[derive(Debug, Clone, bincode::Decode)]
pub struct OpenOk {
    frame_info: ChannelFrameInfo,
    // Is this channel? Pika thinks so but looks like - to me
    pub reserved_1: u16,
}
#[derive(Debug, Clone, bincode::Encode)]
pub struct Flow {
    frame_info: ChannelFrameInfo,
    active: Bits,
}
#[derive(Debug, Clone, bincode::Encode)]
pub struct FlowOk {
    frame_info: ChannelFrameInfo,
    active: Bits,
}
#[derive(Debug, Clone, bincode::Encode)]
pub struct Close {
    frame_info: ChannelFrameInfo,
    reply_code: u16,
    reply_text: ShortString,
    closing_class_id: u16,
    closing_method_id: u16,
    reserved_1: ShortString,
}
#[derive(Debug, Clone, bincode::Encode)]
pub struct CloseOk {
    frame_info: ChannelFrameInfo,
}
