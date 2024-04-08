use crate::Properties;

pub type Bytes = Vec<u8>;
pub type Handler = &'static (dyn Fn(Message) + Send + Sync);
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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
