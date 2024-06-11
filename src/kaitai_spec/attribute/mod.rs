use serde::{Deserialize, Deserializer};

pub enum AttributeType {
    Contents(Vec<u8>),
}

struct Attribute {
    id: String,
    doc: Option<String>,
    type_: AttributeType,
}

impl<'de> Deserialize<'de> for Attribute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Attribute;

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::MapAccess<'de>, {
                todo!()
            }
        }
    }
}

