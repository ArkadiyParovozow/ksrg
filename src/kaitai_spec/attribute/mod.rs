use serde::{de::Error, Deserialize, Deserializer};
use std::collections::hash_map::IntoKeys;
use std::collections::HashMap;
mod enumeration;
#[derive(Debug, PartialEq)]
pub enum AttributeType {
    Enumeration(enumeration::Enumeration),
}
#[derive(Debug, PartialEq)]
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

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a object with correct fields") //todo()
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                {
                    let mut id = None;
                    let mut doc = None;
                    let mut type_ = None;

                    let mut key_values: HashMap<String, String> = HashMap::new();
                    while let Some(key) = map.next_key::<String>()? {
                        match key.as_str() {
                            "id" => {
                                if id.is_some() {
                                    return Err(A::Error::duplicate_field("id"));
                                }
                                id = Some(map.next_value::<String>()?);
                            }
                            "doc" => {
                                if doc.is_some() {
                                    return Err(A::Error::duplicate_field("id"));
                                }
                                doc = Some(map.next_value::<String>()?);
                            }
                            _ => {
                                let key_ref = key.clone();
                                if key_values.contains_key(key_ref.as_str()) {
                                    return Err(A::Error::duplicate_field(""));
                                }
                                key_values.insert(key, map.next_value::<String>()?);
                            }
                        }
                    }
                    let mut key_iter: IntoKeys<String, String> = key_values.clone().into_keys();
                    while let Some(key) = key_iter.next() {
                        match key.as_str() {
                            "enum" => type_ = Some(enumeration::process::<A>(key_values.clone())?),
                            _ => {
                                //todo!()
                            }
                        }
                    }
                    let id: String = id.ok_or_else(|| A::Error::missing_field("id"))?;
                    let type_: AttributeType =
                        type_.ok_or_else(|| A::Error::unknown_variant(id.as_str(), &[""]))?;
                    Ok(Attribute { id, doc, type_ })
                }
            }
        }

        deserializer.deserialize_map(Visitor)
    }
}
