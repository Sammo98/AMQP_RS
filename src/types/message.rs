use crate::Properties;
pub type Bytes = Vec<u8>;

pub struct Message {
    pub bytes: Bytes,
    pub properties: Properties,
    pub additional_info: AdditionalInfo,
}

impl Message {
    pub fn new(bytes: Bytes, properties: Properties, additional_info: AdditionalInfo) -> Self {
        Self {
            bytes,
            properties,
            additional_info,
        }
    }
}

pub struct AdditionalInfo {
    pub delivery_tag: u64,
}

impl AdditionalInfo {
    pub fn new(delivery_tag: u64) -> Self {
        Self { delivery_tag }
    }
}
pub type Handler = &'static (dyn Fn(Message) -> Option<Vec<u8>> + Send + Sync);
