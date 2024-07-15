use serde::de::{self, MapAccess};
use serde::{Deserialize, Deserializer};
use std::fmt;

use super::CommonKeys;
use super::Attribute;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum StringOrByte<'a> {
    String(&'a str),
    Byte(u8),
}

#[derive(Debug)]
pub struct ContentsBytes(pub Vec<u8>);

impl<'de> Deserialize<'de> for ContentsBytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = ContentsBytes;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence of bytes or strings")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ContentsBytes(value.as_bytes().to_vec()))
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

                Ok(ContentsBytes(bytes))
            }
        }
        deserializer.deserialize_any(Visitor)
    }
}

pub fn process<'de, A>(mut common_keys: CommonKeys, mut map: A) -> Result<Attribute, A::Error>
where
    A: MapAccess<'de>,
{
    let ContentsBytes(contents_bytes) = map.next_value()?;

    while let Some(key) = map.next_key::<&str>()? {
        if common_keys.process(key, &mut map)? {
            continue;
        } else if key == "contents" {
            return Err(de::Error::duplicate_field("contents"));
        } else {
            return Err(de::Error::unknown_field(
                key,
                &["id", "doc", "doc-ref", "contents"],
            ));
        }
    }

    Ok(super::Attribute {
        id: common_keys.id.unwrap_or_default(),
        doc: common_keys.doc,
        doc_ref: common_keys.doc_ref,
        type_: super::AttributeType::Contents(contents_bytes),
    })
}
