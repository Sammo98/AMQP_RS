#[derive(Debug, Clone)]
pub enum ConnectionMethodID {
    Start,
    StartOk,
    Secure,
    SecureOk,
    Tune,
    TuneOk,
    Open,
    OpenOk,
    Close,
    CloseOk,
}

impl bincode::Decode for ConnectionMethodID {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        Ok(match u16::decode(decoder)? {
            10 => Self::Start,
            11 => Self::StartOk,
            20 => Self::Secure,
            21 => Self::SecureOk,
            30 => Self::Tune,
            31 => Self::TuneOk,
            40 => Self::Open,
            41 => Self::OpenOk,
            50 => Self::Close,
            51 => Self::CloseOk,
            _ => todo!(),
        })
    }
}

impl bincode::Encode for ConnectionMethodID {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        match self {
            ConnectionMethodID::Start => 10_u16.encode(encoder)?,
            ConnectionMethodID::StartOk => 11_u16.encode(encoder)?,
            ConnectionMethodID::Secure => 20_u16.encode(encoder)?,
            ConnectionMethodID::SecureOk => 21_u16.encode(encoder)?,
            ConnectionMethodID::Tune => 30_u16.encode(encoder)?,
            ConnectionMethodID::TuneOk => 31_u16.encode(encoder)?,
            ConnectionMethodID::Open => 40_u16.encode(encoder)?,
            ConnectionMethodID::OpenOk => 41_u16.encode(encoder)?,
            ConnectionMethodID::Close => 50_u16.encode(encoder)?,
            ConnectionMethodID::CloseOk => 51_u16.encode(encoder)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum ChannelMethodID {
    Open,
    OpenOk,
    Flow,
    FlowOk,
    Close,
    CloseOk,
}

impl bincode::Decode for ChannelMethodID {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        Ok(match u16::decode(decoder)? {
            10 => Self::Open,
            11 => Self::OpenOk,
            20 => Self::Flow,
            21 => Self::FlowOk,
            40 => Self::Close,
            41 => Self::CloseOk,
            _ => todo!(),
        })
    }
}

impl bincode::Encode for ChannelMethodID {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        match self {
            Self::Open => 10_u16.encode(encoder)?,
            Self::OpenOk => 11_u16.encode(encoder)?,
            Self::Flow => 20_u16.encode(encoder)?,
            Self::FlowOk => 21_u16.encode(encoder)?,
            Self::Close => 40_u16.encode(encoder)?,
            Self::CloseOk => 41_u16.encode(encoder)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum ExchangeMethodID {
    Declare,
    DeclareOk,
    Delete,
    DeleteOk,
    Bind,
    BindOk,
    Unbind,
    UnbindOk,
    // Bind?
}

impl bincode::Decode for ExchangeMethodID {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        Ok(match u16::decode(decoder)? {
            10 => Self::Declare,
            11 => Self::DeclareOk,
            20 => Self::Delete,
            21 => Self::DeleteOk,
            30 => Self::Bind,
            31 => Self::Bind,
            40 => Self::Unbind,
            41 => Self::UnbindOk,
            _ => todo!(),
        })
    }
}

impl bincode::Encode for ExchangeMethodID {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        match self {
            Self::Declare => 10_u16.encode(encoder)?,
            Self::DeclareOk => 11_u16.encode(encoder)?,
            Self::Delete => 20_u16.encode(encoder)?,
            Self::DeleteOk => 21_u16.encode(encoder)?,
            Self::Bind => 30_u16.encode(encoder)?,
            Self::BindOk => 31_u16.encode(encoder)?,
            Self::Unbind => 40_u16.encode(encoder)?,
            Self::UnbindOk => 41_u16.encode(encoder)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum QueueMethodID {
    Declare,
    DeclareOk,
    Bind,
    BindOk,
    Purge,
    PurgeOk,
    Delete,
    DeleteOk,
    Unbind,
    UnbindOk,
}

impl bincode::Decode for QueueMethodID {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        Ok(match u16::decode(decoder)? {
            10 => Self::Declare,
            11 => Self::DeclareOk,
            20 => Self::Bind,
            21 => Self::BindOk,
            30 => Self::Purge,
            31 => Self::PurgeOk,
            40 => Self::Delete,
            41 => Self::DeleteOk,
            50 => Self::Unbind,
            51 => Self::UnbindOk,
            _ => todo!(),
        })
    }
}

impl bincode::Encode for QueueMethodID {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        match self {
            Self::Declare => 10_u16.encode(encoder)?,
            Self::DeclareOk => 11_u16.encode(encoder)?,
            Self::Bind => 20_u16.encode(encoder)?,
            Self::BindOk => 21_u16.encode(encoder)?,
            Self::Purge => 30_u16.encode(encoder)?,
            Self::PurgeOk => 31_u16.encode(encoder)?,
            Self::Delete => 40_u16.encode(encoder)?,
            Self::DeleteOk => 41_u16.encode(encoder)?,
            Self::Unbind => 50_u16.encode(encoder)?,
            Self::UnbindOk => 51_u16.encode(encoder)?,
        }
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub enum BasicMethodID {
    QualityOfService,
    QualityOfServiceOk,
    Consume,
    ConsumeOk,
    Cancel,
    CancelOk,
    Publish,
    Return,
    Deliver,
    Get,
    GetOk,
    GetEmpty,
    Ack,
    Reject,
    Recover,
    RecoverOk,
}

impl bincode::Decode for BasicMethodID {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        Ok(match u16::decode(decoder)? {
            10 => Self::QualityOfService,
            11 => Self::QualityOfServiceOk,
            20 => Self::Consume,
            21 => Self::ConsumeOk,
            30 => Self::Cancel,
            31 => Self::CancelOk,
            40 => Self::Publish,
            50 => Self::Return,
            60 => Self::Deliver,
            70 => Self::Get,
            71 => Self::GetOk,
            72 => Self::GetEmpty,
            80 => Self::Ack,
            90 => Self::Reject,
            110 => Self::Recover,
            111 => Self::RecoverOk,
            _ => todo!(),
        })
    }
}

impl bincode::Encode for BasicMethodID {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        match self {
            Self::QualityOfService => 10_u16.encode(encoder)?,
            Self::QualityOfServiceOk => 11_u16.encode(encoder)?,
            Self::Consume => 20_u16.encode(encoder)?,
            Self::ConsumeOk => 21_u16.encode(encoder)?,
            Self::Cancel => 30_u16.encode(encoder)?,
            Self::CancelOk => 31_u16.encode(encoder)?,
            Self::Publish => 40_u16.encode(encoder)?,
            Self::Return => 50_u16.encode(encoder)?,
            Self::Deliver => 60_u16.encode(encoder)?,
            Self::Get => 70_u16.encode(encoder)?,
            Self::GetOk => 71_u16.encode(encoder)?,
            Self::GetEmpty => 72_u16.encode(encoder)?,
            Self::Ack => 80_u16.encode(encoder)?,
            Self::Reject => 90_u16.encode(encoder)?,
            Self::Recover => 110_u16.encode(encoder)?,
            Self::RecoverOk => 111_u16.encode(encoder)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum TransactionMethodId {
    Select,
    SelectOk,
    Commit,
    CommitOk,
    Rollback,
    RollbackOk,
}
impl bincode::Decode for TransactionMethodId {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        Ok(match u16::decode(decoder)? {
            10 => Self::Select,
            11 => Self::SelectOk,
            20 => Self::Commit,
            21 => Self::CommitOk,
            30 => Self::Rollback,
            31 => Self::RollbackOk,
            _ => todo!(),
        })
    }
}

impl bincode::Encode for TransactionMethodId {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        match self {
            TransactionMethodId::Select => 10_u16.encode(encoder)?,
            TransactionMethodId::SelectOk => 11_u16.encode(encoder)?,
            TransactionMethodId::Commit => 20_u16.encode(encoder)?,
            TransactionMethodId::CommitOk => 21_u16.encode(encoder)?,
            TransactionMethodId::Rollback => 30_u16.encode(encoder)?,
            TransactionMethodId::RollbackOk => 31_u16.encode(encoder)?,
        }
        Ok(())
    }
}

bincode::impl_borrow_decode!(ConnectionMethodID);
bincode::impl_borrow_decode!(ChannelMethodID);
bincode::impl_borrow_decode!(QueueMethodID);
bincode::impl_borrow_decode!(BasicMethodID);
bincode::impl_borrow_decode!(ExchangeMethodID);
bincode::impl_borrow_decode!(TransactionMethodId);
