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

impl QualityOfService {
    pub fn new(channel_id: u16, prefetch_size: u32, prefetch_count: u16, global: bool) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel_id,
            size: 0,
        };
        let class_id = ClassID::Basic;
        let method_id = BasicMethodID::QualityOfService;
        let frame_info = BasicFrameInfo {
            header,
            class_id,
            method_id,
        };
        Self {
            frame_info,
            prefetch_size,
            prefetch_count,
            global: (global,).into(),
        }
    }
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
    pub fn new(channel_id: u16, queue_name: &str) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel_id,
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

impl Cancel {
    pub fn new(channel_id: u16, consumer_tag: &str, no_wait: bool) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel_id,
            size: 0,
        };
        let class_id = ClassID::Basic;
        let method_id = BasicMethodID::Cancel;
        let frame_info = BasicFrameInfo {
            header,
            class_id,
            method_id,
        };
        Self {
            frame_info,
            consumer_tag: consumer_tag.into(),
            no_wait: no_wait.into(),
        }
    }
}

#[derive(Debug, Clone, bincode::Decode)]
pub struct CancelOk {
    frame_info: BasicFrameInfo,
    consumer_tag: ShortString,
}

#[derive(Debug, Clone, bincode::Decode)]
pub struct Return {
    frame_info: BasicFrameInfo,
    reply_code: u16,
    reply_text: ShortString,
    exchange_name: ShortString,
    routing_key: ShortString,
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
    pub fn new(
        channel_id: u16,
        exchange_name: &str,
        routing_key: &str,
        mandatory: bool,
        immediate: bool,
    ) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel_id,
            size: 0,
        };
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
pub struct Get {
    frame_info: BasicFrameInfo,
    reserved_1: u16,
    queue_name: ShortString,
    no_ack: Bits,
}

impl Get {
    pub fn new(channel_id: u16, queue_name: &str, no_ack: bool) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel_id,
            size: 0,
        };
        let class_id = ClassID::Basic;
        let method_id = BasicMethodID::Get;
        let frame_info = BasicFrameInfo {
            header,
            class_id,
            method_id,
        };
        Self {
            frame_info,
            reserved_1: RESERVED16,
            queue_name: queue_name.into(),
            no_ack: no_ack.into(),
        }
    }
}

#[derive(Debug, Clone, bincode::Decode)]
pub struct GetOk {
    frame_info: BasicFrameInfo,
    redelivered: Bits,
    delivery_tag: u64,
    exchange_name: ShortString,
    routing_key: ShortString,
    message_count: u32,
}

#[derive(Debug, Clone, bincode::Decode)]
pub struct GetEmpty {
    frame_info: BasicFrameInfo,
    reserved_1: u16,
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
            channel_id: 1,
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

#[derive(Debug, Clone, bincode::Encode)]
pub struct Reject {
    frame_info: BasicFrameInfo,
    delivery_tag: u64,
    requeue: Bits,
}

impl Reject {
    pub fn new(channel_id: u16, delivery_tag: u64, requeue: bool) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel_id,
            size: 0,
        };
        let class_id = ClassID::Basic;
        let method_id = BasicMethodID::Reject;
        let frame_info = BasicFrameInfo {
            header,
            class_id,
            method_id,
        };
        Self {
            frame_info,
            delivery_tag,
            requeue: requeue.into(),
        }
    }
}

#[derive(Debug, Clone, bincode::Encode)]
pub struct Recover {
    frame_info: BasicFrameInfo,
    requeue: Bits,
}

impl Recover {
    pub fn new(channel_id: u16, requeue: bool) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel_id,
            size: 0,
        };
        let class_id = ClassID::Basic;
        let method_id = BasicMethodID::Recover;
        let frame_info = BasicFrameInfo {
            header,
            class_id,
            method_id,
        };
        Self {
            frame_info,
            requeue: requeue.into(),
        }
    }
}

#[derive(Debug, Clone, bincode::Decode)]
pub struct RecoverOk {
    frame_info: BasicFrameInfo,
}
