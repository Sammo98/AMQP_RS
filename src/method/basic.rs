use crate::common::{FrameType, Header};
use crate::constants::{basic_method_id, class_id};
use crate::endec::{Bits, ShortString, Table};
use bincode::{Decode, Encode};

#[derive(Debug, Clone, Encode)]
pub struct Publish {
    header: Header,
    class_id: u16,
    method_id: u16,
    reserved_1: u16,
    exchange_name: ShortString,
    routing_key: ShortString,
    // mandatory, immediate
    bits: Bits,
    frame_end: u8,
}

impl Publish {
    pub fn new(exchange_name: ShortString, routing_key: ShortString, bits: Bits) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel: 1,
            size: 0,
        };
        // Make these enums
        let class_id = class_id::BASIC;
        let method_id = basic_method_id::PUBLISH;
        let frame_end = 0xCE;
        Self {
            header,
            class_id,
            method_id,
            reserved_1: 0,
            exchange_name,
            routing_key,
            bits,
            frame_end,
        }
    }
}

#[derive(Debug, Clone, Encode)]
pub struct Consume {
    header: Header,
    class_id: u16,
    method_id: u16,
    reserved_1: u16,
    queue_name: ShortString,
    consumer_tag: ShortString,
    bits: Bits,
    arguments: Table,
    frame_end: u8,
}

impl Consume {
    pub fn new(queue_name: ShortString, consumer_tag: ShortString, bits: Bits) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel: 1,
            size: 0,
        };
        let class_id = class_id::BASIC;
        let method_id = basic_method_id::CONSUME;
        let frame_end = 0xCE;
        Self {
            header,
            class_id,
            method_id,
            reserved_1: 0,
            queue_name,
            consumer_tag,
            bits,
            arguments: Table(vec![]),
            frame_end,
        }
    }
}
#[derive(Debug, Clone, Decode)]
pub struct Deliver {
    header: Header,
    class_id: u16,
    method_id: u16,
    consumer_tag: ShortString,
    delivery_tag: u64,
    redelivered: bool,
    exchange: ShortString,
    routing: ShortString,
    frame_end: u8,
}
