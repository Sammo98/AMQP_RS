


use bincode::error::EncodeError;
use bincode::{error::DecodeError, Decode, Encode};
use bincode::{impl_borrow_decode};
use std::ops::Deref;

#[derive(Debug, Clone)]
struct Table(Vec<(String, Field)>);

impl Deref for Table {
    type Target = Vec<(String, Field)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Table {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        for (key, value) in self.clone().iter() {
            bytes.push(key.len() as u8); // Key is a short string, push it's length as u8
            bytes.extend_from_slice(key.as_bytes());

            match value {
                Field::SS(s) => {
                    bytes.push('s' as u8);
                    bytes.push(s.len() as u8);
                    bytes.extend_from_slice(s.as_bytes());
                }
                Field::T(t) => {
                    bytes.push('F' as u8);
                    bytes.extend_from_slice(&t.to_bytes());
                }
            }
        }
        let mut length_bytes = (bytes.len() as u32).to_be_bytes().to_vec();
        length_bytes.extend_from_slice(&bytes);
        length_bytes
    }
}

impl_borrow_decode!(Table);

impl Encode for Table {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        let bytes = self.to_bytes();
        for item in bytes.iter() {
            item.encode(encoder)?;
        }
        Ok(())
    }
}

impl Decode for Table {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let length = u32::decode(decoder)? as usize;
        let mut table = vec![];
        let mut parsed: usize = 0;
        while parsed < length {
            let key_length = u8::decode(decoder)?;
            parsed += key_length as usize;

            let mut string_vec = vec![];
            for _ in 0..key_length {
                string_vec.push(u8::decode(decoder)?);
            }
            parsed += string_vec.len();
            let name = String::from_utf8(string_vec).unwrap();

            let field_type = char::decode(decoder).unwrap();
            parsed += 1;

            let val = match field_type {
                's' => {
                    let key_length = u8::decode(decoder)?;
                    parsed += key_length as usize;
                    let mut string_vec = vec![];
                    for _ in 0..key_length {
                        string_vec.push(u8::decode(decoder)?);
                    }
                    parsed += string_vec.len();
                    let name = String::from_utf8(string_vec).unwrap();
                    Field::SS(ShortString(name))
                }
                'F' => {
                    // This is pretty hacky, but once decoding an inner table, we have to convert back to bytes to get the length.
                    let x = Table::decode(decoder)?;
                    parsed += x.to_bytes().len();
                    Field::T(x)
                }
                _ => todo!(),
            };
            table.push((name, val));
        }
        Ok(Self(table))
    }
}

#[derive(Debug, Clone)]
struct ShortString(String);

impl Deref for ShortString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Encode for ShortString {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        // Make more exact, u32 then into vec and so on:
        String::encode(&self, encoder)?;
        Ok(())
    }
}

impl Decode for ShortString {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        // Take exact, u8 then into vec and so on
        let x: String = Decode::decode(decoder).unwrap();
        Ok(Self(x))
    }
}

#[derive(Debug, Clone)]
struct U32(u32);

impl Deref for U32 {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Encode for U32 {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        u32::encode(&self, encoder)?;
        Ok(())
    }
}

impl Decode for U32 {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let x = u32::decode(decoder)?;
        Ok(Self(x))
    }
}
#[derive(Debug, Clone)]
struct U16(u16);

impl Deref for U16 {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Encode for U16 {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        u16::encode(&self, encoder)?;
        Ok(())
    }
}

impl Decode for U16 {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let x = u16::decode(decoder)?;
        Ok(Self(x))
    }
}

//////////////////////////////////////////////////

// Here we need to add all fields under a common enum simply for the table.
// we do not need to implement enc/dec directly here
#[derive(Debug, Clone)]
enum Field {
    SS(ShortString),
    T(Table),
}

/////////////////////////////////////////////

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_table() {
        #[derive(Debug, Encode, Decode)]
        struct TableTest {
            inner: Table,
        }
        let sub_table = Table(vec![("def".into(), Field::SS(ShortString("def".into())))]);
        let original = TableTest {
            inner: Table(vec![("abc".into(), Field::T(sub_table))]),
        };
        let config = bincode::config::standard()
            .with_big_endian()
            .with_fixed_int_encoding();
        let encoded = bincode::encode_to_vec(original, config).unwrap();
        let (_, _): (TableTest, usize) = bincode::decode_from_slice(&encoded, config).unwrap();
    }
}
