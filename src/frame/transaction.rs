use crate::encde::*;

#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
pub struct TransactionFrameInfo {
    header: Header,
    class_id: ClassID,
    method_id: TransactionMethodId,
}

#[derive(Debug, Clone, bincode::Encode)]
pub struct Select {
    frame_info: TransactionFrameInfo,
}

impl Select {
    pub fn new(channel_id: u16) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel_id,
            size: 0,
        };
        let class_id = ClassID::Transaction;
        let method_id = TransactionMethodId::Select;
        let frame_info = TransactionFrameInfo {
            header,
            class_id,
            method_id,
        };
        Self { frame_info }
    }
}

#[derive(Debug, Clone, bincode::Decode)]
pub struct SelectOk {
    frame_info: TransactionFrameInfo,
}

#[derive(Debug, Clone, bincode::Encode)]
pub struct Commit {
    frame_info: TransactionFrameInfo,
}

impl Commit {
    pub fn new(channel_id: u16) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel_id,
            size: 0,
        };
        let class_id = ClassID::Transaction;
        let method_id = TransactionMethodId::Commit;
        let frame_info = TransactionFrameInfo {
            header,
            class_id,
            method_id,
        };
        Self { frame_info }
    }
}

#[derive(Debug, Clone, bincode::Decode)]
pub struct CommitOk {
    frame_info: TransactionFrameInfo,
}

#[derive(Debug, Clone, bincode::Encode)]
pub struct Rollback {
    frame_info: TransactionFrameInfo,
}

impl Rollback {
    pub fn new(channel_id: u16) -> Self {
        let header = Header {
            frame_type: FrameType::Method,
            channel_id,
            size: 0,
        };
        let class_id = ClassID::Transaction;
        let method_id = TransactionMethodId::Rollback;
        let frame_info = TransactionFrameInfo {
            header,
            class_id,
            method_id,
        };
        Self { frame_info }
    }
}

#[derive(Debug, Clone, bincode::Decode)]
pub struct RollbackOk {
    frame_info: TransactionFrameInfo,
}
