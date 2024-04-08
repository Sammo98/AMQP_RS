use crate::encde::*;

#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
struct ExchangeFrameInfo {
    header: Header,
    class_id: ClassID,
    method_id: ExchangeMethodID,
}

#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
pub struct Declare {
    frame_info: ExchangeFrameInfo,
    reserved_1: u16,
    exchange: ShortString,
    exchange_type: ShortString, // Probs an enum
    passive_durable: Bits,      // There are two reserved in here, be careful
    arguments: Table,
}

#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
pub struct DeclareOk {
    frame_info: ExchangeFrameInfo,
    reserved_1: u16,
    exchange: ShortString,
    ifunused_nowait: Bits,
}

#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
pub struct Delete {
    frame_info: ExchangeFrameInfo,
}

#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
pub struct DeleteOk {
    frame_info: ExchangeFrameInfo,
}
