use serde::{
    de::{self, Error, MapAccess},
    Deserialize, Deserializer,
};
use std::fmt;

pub enum AttributeType {
    Contents(Vec<u8>),
}

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
                formatter.write_str("a struct with fields 'id', 'doc', 'doc-ref', and 'contents'")
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
                        "contents" => {
                            return contents::process(common_keys, map);
                        }
                        _ => {
                            return Err(de::Error::unknown_field(key, &["id", "doc", "doc-ref", "contents"]));
                        }
                    }
                }
                Err(de::Error::missing_field("contents"))
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

            _ => Ok(false),
        }
    }
}