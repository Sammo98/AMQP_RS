use super::*;

#[derive(Debug, Clone, Default)]
pub struct Properties {
    content_type: Option<String>,
    content_encoding: Option<String>,
    headers: Option<Vec<(String, Field)>>,
    delivery_mode: Option<u8>, // 1 non-persistent; 2 persistent
    priority: Option<u8>,
    correlation_id: Option<String>,
    reply_to: Option<String>,
    expiration: Option<String>,
    message_id: Option<String>,
    timestamp: Option<u64>,
    message_type: Option<String>,
    user_id: Option<String>,
    app_id: Option<String>,
    cluster_id: Option<String>,
}

impl Properties {
    pub fn builder() -> PropertiesBuilder {
        PropertiesBuilder {
            content_type: None,
            content_encoding: None,
            headers: None,
            delivery_mode: None,
            priority: None,
            correlation_id: None,
            reply_to: None,
            expiration: None,
            message_id: None,
            timestamp: None,
            message_type: None,
            user_id: None,
            app_id: None,
            cluster_id: None,
        }
    }
}

pub struct PropertiesBuilder {
    content_type: Option<String>,
    content_encoding: Option<String>,
    headers: Option<Vec<(String, Field)>>,
    delivery_mode: Option<u8>,
    priority: Option<u8>,
    correlation_id: Option<String>,
    reply_to: Option<String>,
    expiration: Option<String>,
    message_id: Option<String>,
    timestamp: Option<u64>,
    message_type: Option<String>,
    user_id: Option<String>,
    app_id: Option<String>,
    cluster_id: Option<String>,
}

impl PropertiesBuilder {
    pub fn content_type(mut self, content_type: String) -> Self {
        self.content_type = Some(content_type);
        self
    }
    pub fn content_encoding(mut self, content_encoding: String) -> Self {
        self.content_encoding = Some(content_encoding);
        self
    }
    pub fn headers(mut self, headers: Vec<(String, Field)>) -> Self {
        self.headers = Some(headers);
        self
    }
    pub fn delivery_mode(mut self, delivery_mode: u8) -> Self {
        self.delivery_mode = Some(delivery_mode);
        self
    }
    pub fn priority(mut self, priority: u8) -> Self {
        self.priority = Some(priority);
        self
    }
    pub fn correlation_id(mut self, correlation_id: String) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }
    pub fn reply_to(mut self, reply_to: String) -> Self {
        self.reply_to = Some(reply_to);
        self
    }
    pub fn expiration(mut self, expiration: String) -> Self {
        self.expiration = Some(expiration); // This needs to be a string int
        self
    }
    pub fn message_id(mut self, message_id: String) -> Self {
        self.message_id = Some(message_id);
        self
    }
    pub fn timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = Some(timestamp);
        self
    }
    pub fn message_type(mut self, message_type: String) -> Self {
        self.message_type = Some(message_type);
        self
    }
    pub fn user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }
    pub fn app_id(mut self, app_id: String) -> Self {
        self.app_id = Some(app_id);
        self
    }
    pub fn cluster_id(mut self, cluster_id: String) -> Self {
        self.cluster_id = Some(cluster_id);
        self
    }

    pub fn build(self) -> Properties {
        Properties {
            content_type: self.content_type,
            content_encoding: self.content_encoding,
            headers: self.headers,
            delivery_mode: self.delivery_mode,
            priority: self.priority,
            correlation_id: self.correlation_id,
            reply_to: self.reply_to,
            expiration: self.expiration,
            message_id: self.message_id,
            timestamp: self.timestamp,
            message_type: self.message_type,
            user_id: self.user_id,
            app_id: self.app_id,
            cluster_id: self.cluster_id,
        }
    }
}
const CONTENT_TYPE: u16 = 1 << 15;
const CONTENT_ENCODING: u16 = 1 << 14;
const HEADERS: u16 = 1 << 13;
const DELIVERY_MODE: u16 = 1 << 12;
const PRIORITY: u16 = 1 << 11;
const CORRELATION_ID: u16 = 1 << 10;
const REPLY_TO: u16 = 1 << 9;
const EXPIRATION: u16 = 1 << 8;
const MESSAGE_ID: u16 = 1 << 7;
const TIMESTAMP: u16 = 1 << 6;
const MESSAGE_TYPE: u16 = 1 << 5;
const USER_ID: u16 = 1 << 4;
const APP_ID: u16 = 1 << 3;
const CLUSTER_ID: u16 = 1 << 2;

impl bincode::Decode for Properties {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let flags = u16::decode(decoder)?;
        let content_type = match (flags & CONTENT_TYPE) != 0 {
            true => {
                let ShortString(val) = ShortString::decode(decoder)?;
                Some(val)
            }
            false => None,
        };
        let content_encoding = match (flags & CONTENT_ENCODING) != 0 {
            true => {
                let ShortString(val) = ShortString::decode(decoder)?;
                Some(val)
            }
            false => None,
        };
        let headers = match (flags & HEADERS) != 0 {
            true => {
                let Table(val) = Table::decode(decoder)?;
                Some(val)
            }
            false => None,
        };
        let delivery_mode = match (flags & DELIVERY_MODE) != 0 {
            true => Some(u8::decode(decoder)?),
            false => None,
        };
        let priority = match (flags & PRIORITY) != 0 {
            true => Some(u8::decode(decoder)?),
            false => None,
        };
        let correlation_id = match (flags & CORRELATION_ID) != 0 {
            true => {
                let ShortString(val) = ShortString::decode(decoder)?;
                Some(val)
            }
            false => None,
        };
        let reply_to = match (flags & REPLY_TO) != 0 {
            true => {
                let ShortString(val) = ShortString::decode(decoder)?;
                Some(val)
            }
            false => None,
        };
        let expiration = match (flags & EXPIRATION) != 0 {
            true => {
                let ShortString(val) = ShortString::decode(decoder)?;
                Some(val)
            }
            false => None,
        };
        let message_id = match (flags & MESSAGE_ID) != 0 {
            true => {
                let ShortString(val) = ShortString::decode(decoder)?;
                Some(val)
            }
            false => None,
        };
        let timestamp = match (flags & TIMESTAMP) != 0 {
            true => {
                let val = u64::decode(decoder)?;
                Some(val)
            }
            false => None,
        };
        let message_type = match (flags & MESSAGE_TYPE) != 0 {
            true => {
                let ShortString(val) = ShortString::decode(decoder)?;
                Some(val)
            }
            false => None,
        };
        let user_id = match (flags & USER_ID) != 0 {
            true => {
                let ShortString(val) = ShortString::decode(decoder)?;
                Some(val)
            }
            false => None,
        };
        let app_id = match (flags & APP_ID) != 0 {
            true => {
                let ShortString(val) = ShortString::decode(decoder)?;
                Some(val)
            }
            false => None,
        };
        let cluster_id = match (flags & CLUSTER_ID) != 0 {
            true => {
                let ShortString(val) = ShortString::decode(decoder)?;
                Some(val)
            }
            false => None,
        };

        Ok(Self {
            content_type,
            content_encoding,
            headers,
            delivery_mode,
            priority,
            correlation_id,
            reply_to,
            expiration,
            message_id,
            timestamp,
            message_type,
            user_id,
            app_id,
            cluster_id,
        })
    }
}

impl bincode::Encode for Properties {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        let mut flags: u16 = 0;
        if self.content_type.is_some() {
            flags |= CONTENT_TYPE;
        }
        if self.content_encoding.is_some() {
            flags |= CONTENT_ENCODING;
        }
        if self.headers.is_some() {
            flags |= HEADERS;
        }
        if self.delivery_mode.is_some() {
            flags |= DELIVERY_MODE;
        }
        if self.priority.is_some() {
            flags |= PRIORITY;
        }
        if self.correlation_id.is_some() {
            flags |= CORRELATION_ID;
        }
        if self.reply_to.is_some() {
            flags |= REPLY_TO;
        }
        if self.expiration.is_some() {
            flags |= EXPIRATION;
        }
        if self.message_id.is_some() {
            flags |= MESSAGE_ID;
        }
        if self.timestamp.is_some() {
            flags |= TIMESTAMP;
        }
        if self.message_type.is_some() {
            flags |= MESSAGE_TYPE;
        }
        if self.user_id.is_some() {
            flags |= USER_ID;
        }
        if self.app_id.is_some() {
            flags |= APP_ID;
        }
        if self.cluster_id.is_some() {
            flags |= CLUSTER_ID;
        }
        flags.encode(encoder)?;

        if let Some(val) = &self.content_type {
            let val = ShortString(val.clone());
            val.encode(encoder)?;
        }
        if let Some(val) = &self.content_encoding {
            let val = ShortString(val.clone());
            val.encode(encoder)?;
        }
        if let Some(val) = &self.headers {
            let val = Table(val.clone());
            val.encode(encoder)?;
        }
        if let Some(val) = &self.delivery_mode {
            val.encode(encoder)?;
        }
        if let Some(val) = &self.priority {
            val.encode(encoder)?;
        }
        if let Some(val) = &self.correlation_id {
            let val = ShortString(val.clone());
            val.encode(encoder)?;
        }
        if let Some(val) = &self.reply_to {
            let val = ShortString(val.clone());
            val.encode(encoder)?;
        }
        if let Some(val) = &self.expiration {
            let val = ShortString(val.clone());
            val.encode(encoder)?;
        }
        if let Some(val) = &self.message_id {
            let val = ShortString(val.clone());
            val.encode(encoder)?;
        }
        if let Some(val) = &self.timestamp {
            val.encode(encoder)?;
        }
        if let Some(val) = &self.message_type {
            let val = ShortString(val.clone());
            val.encode(encoder)?;
        }
        if let Some(val) = &self.user_id {
            let val = ShortString(val.clone());
            val.encode(encoder)?;
        }
        if let Some(val) = &self.app_id {
            let val = ShortString(val.clone());
            val.encode(encoder)?;
        }
        if let Some(val) = &self.cluster_id {
            let val = ShortString(val.clone());
            val.encode(encoder)?;
        }

        Ok(())
    }
}
bincode::impl_borrow_decode!(Properties);
