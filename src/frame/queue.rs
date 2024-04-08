use crate::encde::*;

#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
struct QueueFrameInfo {
    header: Header,
    class_id: ClassID,
    method_id: QueueMethodID,
}

#[derive(Debug, Clone, bincode::Encode)]
pub struct Declare {
    frame_info: QueueFrameInfo,
    reserved_1: u16,
    queue: ShortString,
    // passive, durable, exclusive, auto_delete, no_wait
    bits: Bits,
    arguments: Table,
}

impl Declare {
    pub fn new(queue: ShortString, bits: Bits) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel: 1,
            size: 0,
        };
        let class_id = ClassID::Queue;
        let method_id = QueueMethodID::Declare;
        let frame_info = QueueFrameInfo {
            header,
            class_id,
            method_id,
        };
        Self {
            frame_info,
            reserved_1: 0,
            queue,
            bits,
            arguments: Table(vec![]),
        }
    }
}

#[derive(Debug, Clone, bincode::Decode)]
pub struct DeclareOk {
    frame_info: QueueFrameInfo,
    _queue_name: ShortString,
    pub message_count: u32,
    _consumer_count: u32,
}
