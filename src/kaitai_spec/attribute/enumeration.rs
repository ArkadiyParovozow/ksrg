use super::{Attribute, ContextNoContents, ContextTypeAttributes};
use either::Either;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Enumeration {
    pub name: String,
    pub type_: super::common::Integer,
}

pub fn try_build<'de, A>(
    context: ContextNoContents,
) -> Either<Result<Attribute, A::Error>, ContextTypeAttributes>
where
    A: serde::de::MapAccess<'de>,
{
    match context.type_attributes.is_some() {
        true => return Either::Left(build_attribute::<A>(context)),
        false => {
            return Either::Right(ContextTypeAttributes {
                string_keys: context.string_keys,
                size: context.size,
                type_: context.type_,
                type_attributes: context.type_attributes.unwrap().right(),
            })
        }
    }
}

fn build_attribute<'de, A>(context: ContextNoContents) -> Result<Attribute, A::Error>
where
    A: serde::de::MapAccess<'de>,
{
    let name: String = context.type_attributes.unwrap().left().unwrap().0;
    let mut keys: HashMap<&str, String> = context.string_keys.clone();
    let id: String = keys
        .remove("id")
        .ok_or_else(|| serde::de::Error::missing_field("type"))?;
    let doc: Option<String> = keys.remove("doc");
    let doc_ref: Option<String> = keys.remove("doc_ref");
    for key in keys.into_keys() {
        return Err(serde::de::Error::unknown_field(key, &["id", "doc", "type"]));
    }

    let type_unchecked: String = context
        .type_
        .ok_or_else(|| serde::de::Error::missing_field("type"))?;
    let type_: super::common::Integer = super::common::type_parse::<A>(type_unchecked)?;

    Ok(Attribute {
        id,
        doc,
        doc_ref,
        type_: super::AttributeType::Enumeration(Enumeration { name, type_ }),
    })
}
