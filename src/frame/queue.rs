use crate::endec::*;

#[derive(Debug, Clone, bincode::Encode)]
pub struct Declare {
    header: Header,
    class_id: ClassID,
    method_id: QueueMethodId,
    reserved_1: u16,
    queue: ShortString,
    // passive, durable, exclusive, auto_delete, no_wait
    bits: Bits,
    arguments: Table,
    frame_end: u8,
}

impl Declare {
    pub fn new(queue: ShortString, bits: Bits) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel: 1,
            size: 0,
        };
        let class_id = ClassID::Queue;
        let method_id = QueueMethodId::Declare;
        let frame_end = 0xCE;
        Self {
            header,
            class_id,
            method_id,
            reserved_1: 0,
            queue,
            bits,
            arguments: Table(vec![]),
            frame_end,
        }
    }
}
