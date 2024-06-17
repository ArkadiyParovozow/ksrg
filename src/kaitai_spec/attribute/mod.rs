use serde::{Deserialize, Deserializer};
use serde::de::{self, MapAccess, Visitor};
use std::fmt;

#[derive(Debug)]
pub enum AttributeType {
    Contents(Vec<u8>),
}

#[derive(Debug)]
struct Attribute {
    id: Option<String>,
    type_: AttributeType,
}

struct State {
    id: Option<String>,
}


impl<'de> Deserialize<'de> for Attribute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AttributeVisitor;

        impl<'de> Visitor<'de> for AttributeVisitor {
            type Value = Attribute;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a struct with fields 'id' and 'contents'")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {

                let mut state = State { id: None };

                while let Some(key) = map.next_key::<&str>()? {
                    match key {
                        "id" => {
                            if state.id.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }
                            state.id = Some(map.next_value()?);
                        }
                        "contents" => {
                            return contents::process(state, map);
                        }
                        _ => {
                            let _: de::IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Err(de::Error::missing_field("contents"))
            }
        }

        deserializer.deserialize_map(AttributeVisitor)
    }
}
