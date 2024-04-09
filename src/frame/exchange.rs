use crate::encde::*;

#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
struct ExchangeFrameInfo {
    header: Header,
    class_id: ClassID,
    method_id: ExchangeMethodID,
}

#[derive(Debug, Clone, bincode::Encode)]
pub struct Declare {
    frame_info: ExchangeFrameInfo,
    reserved_1: u16,
    exchange: ShortString,
    exchange_type: ExchangeType,
    passive_durable: Bits, // There are two reserved in here, be careful
    arguments: Table,
}

impl Declare {
    pub fn new(exchange: String, exchange_type: ExchangeType) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel: 1,
            size: 0,
        };
        let class_id = ClassID::Exchange;
        let method_id = ExchangeMethodID::Declare;
        let frame_info = ExchangeFrameInfo {
            header,
            class_id,
            method_id,
        };
        Self {
            frame_info,
            reserved_1: RESERVED16,
            exchange: ShortString(exchange),
            exchange_type,
            passive_durable: Bits(vec![]),
            arguments: Table(vec![]),
        }
    }
}

#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
pub struct DeclareOk {
    frame_info: ExchangeFrameInfo,
}

#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
pub struct Delete {
    frame_info: ExchangeFrameInfo,
    reserved_1: u16,
    exchange: ShortString,
    ifunused_nowait: Bits,
}

impl Delete {
    pub fn new(exchange: String) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel: 1,
            size: 0,
        };
        let class_id = ClassID::Exchange;
        let method_id = ExchangeMethodID::Delete;
        let frame_info = ExchangeFrameInfo {
            header,
            class_id,
            method_id,
        };
        Self {
            frame_info,
            reserved_1: RESERVED16,
            exchange: ShortString(exchange),
            ifunused_nowait: Bits(vec![]),
        }
    }
}

#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
pub struct DeleteOk {
    frame_info: ExchangeFrameInfo,
}

#[derive(Debug, Clone, bincode::Encode)]
pub struct Bind {
    frame_info: ExchangeFrameInfo,
    reserved_1: u16,
    destination: ShortString,
    source: ShortString,
    routing_key: ShortString,
    no_wait: Bits,
    arguments: Table,
}

impl Bind {
    pub fn new(destination: &str, source: &str, routing_key: &str, no_wait: bool) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel: 1,
            size: 0,
        };
        let class_id = ClassID::Exchange;
        let method_id = ExchangeMethodID::Bind;
        let frame_info = ExchangeFrameInfo {
            header,
            class_id,
            method_id,
        };
        Self {
            frame_info,
            reserved_1: RESERVED16,
            destination: ShortString(destination.into()),
            source: ShortString(source.into()),
            routing_key: ShortString(routing_key.into()),
            no_wait: Bits(vec![no_wait.into()]),
            arguments: Table(vec![]),
        }
    }
}
#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
pub struct BindOk {
    frame_info: ExchangeFrameInfo,
}

#[derive(Debug, Clone, bincode::Encode)]
pub struct Unbind {
    frame_info: ExchangeFrameInfo,
    reserved_1: u16,
    destination: ShortString,
    source: ShortString,
    routing_key: ShortString,
    no_wait: Bits,
    arguments: Table,
}

impl Unbind {
    pub fn new(destination: &str, source: &str, routing_key: &str, no_wait: bool) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel: 1,
            size: 0,
        };
        let class_id = ClassID::Exchange;
        let method_id = ExchangeMethodID::Unbind;
        let frame_info = ExchangeFrameInfo {
            header,
            class_id,
            method_id,
        };
        Self {
            frame_info,
            reserved_1: RESERVED16,
            destination: ShortString(destination.into()),
            source: ShortString(source.into()),
            routing_key: ShortString(routing_key.into()),
            no_wait: Bits(vec![no_wait.into()]),
            arguments: Table(vec![]),
        }
    }
}

#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
pub struct UnbindOk {
    frame_info: ExchangeFrameInfo,
}
