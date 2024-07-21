#[cfg(test)]
mod tests;
use serde::{
    de::{self, Error, MapAccess},
    Deserialize, Deserializer,
};
use std::fmt;

mod common_types;
mod contents;
mod enumeration;
mod integers;

#[derive(Debug, PartialEq)]
pub enum AttributeType {
    Contents(Vec<u8>),
    Enumeration(enumeration::Enumeration),
    Integer(common_types::IntType),
}

#[derive(Debug, PartialEq)]
struct Attribute {
    id: String,
    doc: Option<String>,
    doc_ref: Option<String>,
    type_: AttributeType,
}

impl<'de> Deserialize<'de> for Attribute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AttributeVisitor;

        impl<'de> de::Visitor<'de> for AttributeVisitor {
            type Value = Attribute;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a struct with fields such 'id', 'doc', 'doc-ref'")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut common_keys = CommonKeys::default();
                while let Some(key) = map.next_key::<&str>()? {
                    if common_keys.process(key, &mut map)? {
                        continue;
                    }
                    match key {
                        "enum" => return enumeration::process(common_keys, map),
                        "contents" => {
                            return contents::process(common_keys, map);
                        }
                        _ => {
                            return Err(de::Error::unknown_field(
                                key,
                                &["id", "doc", "doc-ref", "contents", "enum"],
                            ));
                        }
                    }
                }
                return integers::process(common_keys, map);
            }
        }

        deserializer.deserialize_map(AttributeVisitor)
    }
}

#[derive(Debug, Default, Clone)]
struct CommonKeys {
    id: Option<String>,
    doc: Option<String>,
    doc_ref: Option<String>,
    type_: Option<String>,
}

impl CommonKeys {
    fn process<'de, A>(&mut self, key: &str, map: &mut A) -> Result<bool, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        match key {
            "id" => match self.id {
                Some(_) => Err(A::Error::duplicate_field("id")),
                None => {
                    self.id = Some(map.next_value::<String>()?);

                    Ok(true)
                }
            },

            "doc" => match self.doc {
                Some(_) => Err(A::Error::duplicate_field("doc")),
                None => {
                    self.doc = Some(map.next_value::<String>()?);

                    Ok(true)
                }
            },

            "doc-ref" => match self.doc_ref {
                Some(_) => Err(A::Error::duplicate_field("doc-ref")),
                None => {
                    self.doc_ref = Some(map.next_value::<String>()?);

                    Ok(true)
                }
            },

            "type" => match self.type_ {
                Some(_) => Err(A::Error::duplicate_field("type")),
                None => {
                    self.type_ = Some(map.next_value::<String>()?);

                    Ok(true)
                }
            },

            _ => Ok(false),
        }
    }
}
