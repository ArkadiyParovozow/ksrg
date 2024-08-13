use super::{
    common::{type_parse, Integer},
    Attribute, AttributeType, ContextTypeAttributes, KEY_DOC, KEY_DOC_REF, KEY_ID,
};
use either::Either;
use serde::de::{Error, MapAccess};

pub fn try_build<'de, A>(
    context: ContextTypeAttributes,
) -> Either<Result<Attribute, A::Error>, ContextTypeAttributes>
where
    A: MapAccess<'de>,
{
    match context.type_attributes {
        Some(_) => return Either::Right(context),
        None => {}
    };
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
        type_: AttributeType::Integer(type_),
    }));
}
