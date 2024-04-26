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
    bits: Bits,
    arguments: Table,
}

impl Declare {
    pub fn new(
        channel_id: u16,
        queue: &str,
        passive: bool,
        durable: bool,
        exclusive: bool,
        auto_delete: bool,
        no_wait: bool,
    ) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel_id,
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
            reserved_1: RESERVED16,
            queue: queue.into(),
            bits: (passive, durable, exclusive, auto_delete, no_wait).into(),
            arguments: Table::default(),
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

#[derive(Debug, Clone, bincode::Encode)]
pub struct Bind {
    frame_info: QueueFrameInfo,
    reserved_1: u16,
    queue: ShortString,
    exchange: ShortString,
    routing_key: ShortString,
    no_wait: Bits,
    arguments: Table,
}

impl Bind {
    pub fn new(queue: &str, exchange: &str, routing_key: &str, no_wait: bool) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel_id: 1,
            size: 0,
        };
        let class_id = ClassID::Queue;
        let method_id = QueueMethodID::Bind;
        let frame_info = QueueFrameInfo {
            header,
            class_id,
            method_id,
        };
        Self {
            frame_info,
            reserved_1: RESERVED16,
            queue: queue.into(),
            exchange: exchange.into(),
            routing_key: routing_key.into(),
            no_wait: (no_wait,).into(),
            arguments: Table::default(),
        }
    }
}

#[derive(Debug, Clone, bincode::Decode)]
pub struct BindOk {
    frame_info: QueueFrameInfo,
}

#[derive(Debug, Clone, bincode::Encode)]
pub struct Unbind {
    frame_info: QueueFrameInfo,
    reserved_1: u16,
    queue: ShortString,
    exchange: ShortString,
    routing_key: ShortString,
    arguments: Table,
}

impl Unbind {
    pub fn new(queue: &str, exchange: &str, routing_key: &str) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel_id: 1,
            size: 0,
        };
        let class_id = ClassID::Queue;
        let method_id = QueueMethodID::Unbind;
        let frame_info = QueueFrameInfo {
            header,
            class_id,
            method_id,
        };
        Self {
            frame_info,
            reserved_1: RESERVED16,
            queue: queue.into(),
            exchange: exchange.into(),
            routing_key: routing_key.into(),
            arguments: Table::default(),
        }
    }
}

#[derive(Debug, Clone, bincode::Decode)]
pub struct UnbindOk {
    frame_info: QueueFrameInfo,
}
#[derive(Debug, Clone, bincode::Encode)]
pub struct Purge {
    frame_info: QueueFrameInfo,
    reserved_1: u16,
    queue: ShortString,
    no_wait: Bits,
}

impl Purge {
    pub fn new(queue: &str, no_wait: bool) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel_id: 1,
            size: 0,
        };
        let class_id = ClassID::Queue;
        let method_id = QueueMethodID::Purge;
        let frame_info = QueueFrameInfo {
            header,
            class_id,
            method_id,
        };
        Self {
            frame_info,
            reserved_1: RESERVED16,
            queue: queue.into(),
            no_wait: (no_wait,).into(),
        }
    }
}

#[derive(Debug, Clone, bincode::Decode)]
pub struct PurgeOk {
    frame_info: QueueFrameInfo,
    message_count: u32,
}
#[derive(Debug, Clone, bincode::Encode)]
pub struct Delete {
    frame_info: QueueFrameInfo,
    reserved_1: u16,
    queue: ShortString,
    ifunused_ifempty_nowait: Bits,
}

impl Delete {
    pub fn new(queue: &str, if_unused: bool, if_empty: bool, no_wait: bool) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel_id: 1,
            size: 0,
        };
        let class_id = ClassID::Queue;
        let method_id = QueueMethodID::Delete;
        let frame_info = QueueFrameInfo {
            header,
            class_id,
            method_id,
        };
        Self {
            frame_info,
            reserved_1: RESERVED16,
            queue: queue.into(),
            ifunused_ifempty_nowait: (if_unused, if_empty, no_wait).into(),
        }
    }
}
#[derive(Debug, Clone, bincode::Decode)]
pub struct DeleteOk {
    frame_info: QueueFrameInfo,
    message_count: u32,
}
