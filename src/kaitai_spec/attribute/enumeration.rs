use super::{Attribute, ContextNoContents, ContextTypeAttributes};
use either::Either;
use serde::de::Error;
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
    let enumeration = match context.type_attributes {
        Some(Either::Left(enumeration)) => enumeration,
        Some(Either::Right(value)) => {
            return Either::Right(ContextTypeAttributes {
                string_keys: context.string_keys,
                size: context.size,
                type_: context.type_,
                type_attributes: Some(value),
            })
        }
        None => {
            return Either::Right(ContextTypeAttributes {
                string_keys: context.string_keys,
                size: context.size,
                type_: context.type_,
                type_attributes: None,
            })
        }
    };
    return Either::Left(build_attribute::<A>(context.string_keys, enumeration.0, context.type_));
}

fn build_attribute<'de, A>(
    mut keys: HashMap<&str, String>,
    name: String,
    type_unchecked: Option<String>
) -> Result<Attribute, A::Error>
where
    A: serde::de::MapAccess<'de>,
{
    let id: String = keys
        .remove("id")
        .ok_or_else(|| Error::missing_field("type"))?;
    let doc: Option<String> = keys.remove("doc");
    let doc_ref: Option<String> = keys.remove("doc_ref");
    for key in keys.into_keys() {
        return Err(Error::unknown_field(key, &["id", "doc", "type"]));
    }

    let type_unchecked: String = type_unchecked.ok_or_else(|| Error::missing_field("type"))?;
    let type_: super::common::Integer = super::common::type_parse::<A>(&type_unchecked)?;

    Ok(Attribute {
        id,
        doc,
        doc_ref,
        type_: super::AttributeType::Enumeration(Enumeration { name, type_ }),
    })
}
