use either::Either;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Enumeration {
    pub name: String,
    pub type_: super::common_types::IntType,
}

pub fn try_build<'de, A>(context: super::ContextNoContents) -> Result<super::Attribute, A::Error>
where
    A: serde::de::MapAccess<'de>,
{
    let mut keys: HashMap<&str, String> = context.string_keys.clone();
    let id: String = keys
        .remove("id")
        .ok_or_else(|| serde::de::Error::missing_field("type"))?;
    let doc: Option<String> = keys.remove("id");
    let doc_ref: Option<String> = keys.remove("doc_ref");
    for key in keys.into_keys() {
        return Err(serde::de::Error::unknown_field(key, &["id", "doc", "type"]));
    }

    let type_unchecked = context
        .type_
        .ok_or_else(|| serde::de::Error::missing_field("type"))?;
    let type_: super::common_types::IntType = super::common_types::type_parse::<A>(type_unchecked)?;

    let name: String = context.type_attributes.unwrap().left().unwrap().0; //TODO

    Ok(super::Attribute {
        id,
        doc,
        doc_ref,
        type_: super::AttributeType::Enumeration(Enumeration { name, type_ }),
    })
}
