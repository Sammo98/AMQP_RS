use crate::endec::*;

#[derive(Debug, Clone, bincode::Encode)]
pub struct Publish {
    header: Header,
    class_id: ClassID,
    method_id: BasicMethodId,
    reserved_1: u16,
    exchange_name: ShortString,
    routing_key: ShortString,
    bits: Bits, // mandatory, immediate
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
        let class_id = ClassID::Basic;
        let method_id = BasicMethodId::Publish;
        let frame_end = FRAME_END;
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

#[derive(Debug, Clone, bincode::Encode)]
pub struct Consume {
    header: Header,
    class_id: ClassID,
    method_id: BasicMethodId,
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
        let class_id = ClassID::Basic;
        let method_id = BasicMethodId::Consume;
        let frame_end = FRAME_END;
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
#[derive(Debug, Clone, bincode::Decode)]
pub struct ConsumeOk {
    header: Header,
    class_id: ClassID,
    method_id: BasicMethodId,
    consumer_tag: ShortString,
    frame_end: u8,
}

#[derive(Debug, Clone, bincode::Decode)]
pub struct Deliver {
    header: Header,
    class_id: ClassID,
    method_id: BasicMethodId,
    consumer_tag: ShortString,
    pub delivery_tag: u64,
    redelivered: bool,
    exchange: ShortString,
    routing: ShortString,
    frame_end: u8,
}

#[derive(Debug, Clone, bincode::Encode)]
pub struct Ack {
    header: Header,
    class_id: ClassID,
    method_id: BasicMethodId,
    delivery_tag: u64,
    multiple: u8,
    frame_end: u8,
}

impl Ack {
    pub fn new(delivery_tag: u64) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel: 1,
            size: 0,
        };
        let class_id = ClassID::Basic;
        let method_id = BasicMethodId::Ack;
        let multiple = 0;
        let frame_end = FRAME_END;

        Self {
            header,
            class_id,
            method_id,
            delivery_tag,
            multiple,
            frame_end,
        }
    }
}
