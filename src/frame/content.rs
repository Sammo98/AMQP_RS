use crate::encde::*;

#[derive(Debug, Clone, bincode::Decode, bincode::Encode)]
pub struct Content {
    header: Header,
    class_id: ClassID,
    weight: u16,
    size: u64,
    pub properties: Properties,
}

impl Content {
    pub fn new(size: u64, properties: Properties) -> Self {
        let header = Header {
            frame_type: FrameType::Header,
            channel: 1,
            size: 0,
        };
        let class_id = ClassID::Basic;
        let weight = 0;
        Self {
            header,
            class_id,
            weight,
            size,
            properties,
        }
    }
}
