use crate::encde::*;

#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
struct BasicFrameInfo {
    header: Header,
    class_id: ClassID,
    method_id: BasicMethodID,
}

#[derive(Debug, Clone, bincode::Encode)]
pub struct QualityOfService {
    frame_info: BasicFrameInfo,
    prefetch_size: u32,
    prefetch_count: u16,
    global: Bits,
}

#[derive(Debug, Clone, bincode::Decode)]
pub struct QualityOfServiceOk {
    frame_info: BasicFrameInfo,
}

#[derive(Debug, Clone, bincode::Encode)]
pub struct Consume {
    frame_info: BasicFrameInfo,
    reserved_1: u16,
    queue_name: ShortString,
    consumer_tag: ShortString,
    bits: Bits,
    arguments: Table,
}

impl Consume {
    pub fn new(queue_name: &str) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel: 1,
            size: 0,
        };
        let class_id = ClassID::Basic;
        let method_id = BasicMethodID::Consume;
        let frame_info = BasicFrameInfo {
            header,
            class_id,
            method_id,
        };
        Self {
            frame_info,
            reserved_1: RESERVED16,
            queue_name: queue_name.into(),
            consumer_tag: ShortString("abc".into()),
            bits: Bits::default(),
            arguments: Table::default(),
        }
    }
}
#[derive(Debug, Clone, bincode::Decode)]
pub struct ConsumeOk {
    frame_info: BasicFrameInfo,
    consumer_tag: ShortString,
}

#[derive(Debug, Clone, bincode::Encode)]
pub struct Cancel {
    frame_info: BasicFrameInfo,
    consumer_tag: ShortString,
    no_wait: Bits,
}

#[derive(Debug, Clone, bincode::Encode)]
pub struct Publish {
    frame_info: BasicFrameInfo,
    reserved_1: u16,
    exchange_name: ShortString,
    routing_key: ShortString,
    bits: Bits, // mandatory, immediate
}

impl Publish {
    pub fn new(exchange_name: &str, routing_key: &str, mandatory: bool, immediate: bool) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel: 1,
            size: 0,
        };
        // Make these enums
        let class_id = ClassID::Basic;
        let method_id = BasicMethodID::Publish;
        let frame_info = BasicFrameInfo {
            header,
            class_id,
            method_id,
        };
        Self {
            frame_info,
            reserved_1: RESERVED16,
            exchange_name: exchange_name.into(),
            routing_key: routing_key.into(),
            bits: (mandatory, immediate).into(),
        }
    }
}

#[derive(Debug, Clone, bincode::Decode)]
pub struct Deliver {
    frame_info: BasicFrameInfo,
    consumer_tag: ShortString,
    pub delivery_tag: u64,
    redelivered: bool,
    exchange: ShortString,
    routing: ShortString,
}

#[derive(Debug, Clone, bincode::Encode)]
pub struct Ack {
    frame_info: BasicFrameInfo,
    delivery_tag: u64,
    multiple: u8,
}

impl Ack {
    pub fn new(delivery_tag: u64) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel: 1,
            size: 0,
        };
        let class_id = ClassID::Basic;
        let method_id = BasicMethodID::Ack;
        let multiple = 0;
        let frame_info = BasicFrameInfo {
            header,
            class_id,
            method_id,
        };

        Self {
            frame_info,
            delivery_tag,
            multiple,
        }
    }
}
