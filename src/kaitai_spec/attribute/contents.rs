use either::Either;
use serde::de::{self, MapAccess};
use serde::{Deserialize, Deserializer};
use std::fmt;

use super::{Attribute, Context, ContextNoContents};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum StringOrByte<'a> {
    String(&'a str),
    Byte(u8),
}

#[derive(Debug)]
pub struct Bytes(pub Vec<u8>);

impl<'de> Deserialize<'de> for Bytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Bytes;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence of bytes or strings")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Bytes(value.as_bytes().to_vec()))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut bytes = Vec::with_capacity(1_000);
                while let Some(string_or_byte) = seq.next_element::<StringOrByte>()? {
                    match string_or_byte {
                        StringOrByte::Byte(b) => bytes.push(b),
                        StringOrByte::String(string) => {
                            bytes.extend_from_slice(string.as_bytes());
                        }
                    }
                }

                Ok(Bytes(bytes))
            }
        }
        deserializer.deserialize_any(Visitor)
    }
}

pub fn try_build<'de, A: MapAccess<'de>>(context: Context) -> Either<Result<Attribute, A::Error>, ContextNoContents> {
    todo!()
}
