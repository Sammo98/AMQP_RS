use crate::endec::{LongString, ShortString};

#[derive(Debug, Clone)]
pub struct Table(pub Vec<(String, Field)>);

impl std::ops::Deref for Table {
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
                    bytes.push(b's');
                    bytes.push(s.len() as u8);
                    bytes.extend_from_slice(s.as_bytes());
                }
                Field::LS(s) => {
                    bytes.push(b'S');
                    bytes.extend_from_slice(&(s.len() as u32).to_be_bytes());
                    bytes.extend_from_slice(s.as_bytes());
                }
                Field::T(t) => {
                    bytes.push(b'F');
                    bytes.extend_from_slice(&t.to_bytes());
                }
                Field::Bool(b) => {
                    bytes.push(b't');
                    bytes.push(*b as u8);
                }
            }
        }
        let mut length_bytes = (bytes.len() as u32).to_be_bytes().to_vec();
        length_bytes.extend_from_slice(&bytes);
        length_bytes
    }
}

bincode::impl_borrow_decode!(Table);

impl bincode::Encode for Table {
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

impl bincode::Decode for Table {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let length = u32::decode(decoder)? as usize;
        let mut table = vec![];
        let mut parsed: usize = 0;
        while parsed < length {
            let key_length = u8::decode(decoder)?;
            parsed += 1;

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
                    parsed += 1;
                    let mut string_vec = vec![];
                    for _ in 0..key_length {
                        string_vec.push(u8::decode(decoder)?);
                    }
                    parsed += string_vec.len();
                    let name = String::from_utf8(string_vec).unwrap();
                    Field::SS(ShortString(name))
                }
                'S' => {
                    let key_length = u32::decode(decoder)?;
                    parsed += 4;
                    let mut string_vec = vec![];
                    for _ in 0..key_length {
                        string_vec.push(u8::decode(decoder)?);
                    }
                    parsed += string_vec.len();
                    let name = String::from_utf8(string_vec).unwrap();
                    Field::LS(LongString(name))
                }
                'F' => {
                    // This is pretty hacky, but once decoding an inner table, we have to convert back to bytes to get the length.
                    let x = Table::decode(decoder)?;
                    parsed += x.to_bytes().len();
                    Field::T(x)
                }
                't' => {
                    let x = bool::decode(decoder)?;
                    parsed += 1;
                    Field::Bool(x)
                }

                _ => {
                    println!("{field_type:?} not supported yet");
                    todo!();
                }
            };

            table.push((name, val));
        }

        Ok(Self(table))
    }
}

//////////////////////////////////////////////////
// Here we need to add all fields under a common enum simply for the table.
// we do not need to implement enc/dec directly here
#[derive(Debug, Clone)]
pub enum Field {
    SS(ShortString),
    LS(LongString),
    T(Table),
    Bool(bool),
}

/////////////////////////////////////////////

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_table() {
        #[derive(Debug, bincode::Encode, bincode::Decode)]
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
