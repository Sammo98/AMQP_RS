use crate::common::{FrameType, Header};
use crate::constants::class_id::QUEUE;
use crate::constants::queue_method_id::DECLARE;
use crate::endec::{Bits, ShortString, Table};
use bincode::Encode;

#[derive(Debug, Clone, Encode)]
pub struct Declare {
    header: Header,
    class_type: u16,
    method_type: u16,
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
        let class_type = QUEUE;
        let method_type = DECLARE;
        let frame_end = 0xCE;
        Self {
            header,
            class_type,
            method_type,
            reserved_1: 0,
            queue,
            bits,
            arguments: Table(vec![]),
            frame_end,
        }
    }
}
