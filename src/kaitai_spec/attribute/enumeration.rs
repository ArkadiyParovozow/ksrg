use super::{
    common::{type_parse, Integer},
    Attribute, AttributeType, ContextNoContents, ContextTypeAttributes, KEY_ID, KEY_DOC, KEY_DOC_REF
};
use either::Either;
use serde::de::{Error, MapAccess};

#[derive(Debug, PartialEq)]
pub struct Enumeration {
    pub name: String,
    pub type_: Integer,
}

pub fn try_build<'de, A>(
    context: ContextNoContents,
) -> Either<Result<Attribute, A::Error>, ContextTypeAttributes>
where
    A: MapAccess<'de>,
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
    let name: String = enumeration.0;
    let mut keys = context.string_keys;
    let id: Option<String> = keys.remove(KEY_ID);
    let doc: Option<String> = keys.remove(KEY_DOC);
    let doc_ref: Option<String> = keys.remove(KEY_DOC_REF);
    for key in keys.into_keys() {
        return Either::Left(Err(Error::unknown_field(key, &[KEY_ID, KEY_DOC, "type"])));
    }

    let type_unchecked: String = match context.type_ {
        Some(type_) => type_,
        None => return Either::Left(Err(Error::missing_field("type"))),
    };
    let type_: Integer = match type_parse::<A>(&type_unchecked) {
        Ok(type_) => type_,
        Err(err) => return Either::Left(Err(err)),
    };
    return Either::Left(Ok(Attribute {
        id,
        doc,
        doc_ref,
        type_: AttributeType::Enumeration(Enumeration { name, type_ }),
    }));
}
