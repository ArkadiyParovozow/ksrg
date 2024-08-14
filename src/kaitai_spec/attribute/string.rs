use either::Either;
use serde::de::{self, MapAccess, Error};

use super::{Attribute, Size, ContextTypeAttributes, AttributeType, TypeAttributes, KEY_ID, KEY_DOC, KEY_DOC_REF};

#[derive(Debug, PartialEq)]
pub struct StringType {
    pub value: String,
    pub terminator: Option<u8>,
    pub size: Option<Size>,
    pub consume: Option<bool>,
    pub include: Option<bool>,
    pub encoding: Option<String>,
}

pub fn try_build<'de, A: MapAccess<'de>>(mut context: ContextTypeAttributes) -> Either<Result<Attribute, A::Error>, ContextTypeAttributes> {
    use Either::*;
    
    if context.type_ == Some("strz".to_string()) || context.type_ == Some("str".to_string()) {
        let type_attrs: TypeAttributes = match context.type_attributes {
            Some(type_attrs) => type_attrs,
            None => return Right(ContextTypeAttributes {
                string_keys: context.string_keys,
                size: context.size,
                type_: context.type_,
                type_attributes: None,
            }),
        };
        
        let string_type = StringType {
            value: if context.type_ == Some("strz".to_string()) { "strz".to_string() } else { "str".to_string() },
            terminator: if context.type_ == Some("strz".to_string()) { Some(0) } else { type_attrs.terminator },
            size: context.size,
            consume: type_attrs.consume,
            include: type_attrs.include,
            encoding: type_attrs.encoding,
        };
    
        let id: Option<String> = context.string_keys.remove(KEY_ID);
        let doc: Option<String> = context.string_keys.remove(KEY_DOC);
        let doc_ref: Option<String> = context.string_keys.remove(KEY_DOC_REF);

        for key in context.string_keys.into_keys() {
            return Left(Err(Error::unknown_field(key, &[KEY_ID, KEY_DOC, "type"])));
        }

        return Left(Ok(Attribute {
            id,
            doc,
            doc_ref,
            type_: AttributeType::String(string_type),
        }));
    } else {
        return Left(Err(de::Error::custom("Unexpected type")));
    }
}
