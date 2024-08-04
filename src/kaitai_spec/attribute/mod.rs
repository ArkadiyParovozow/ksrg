#[cfg(test)]
mod tests;
use either::Either;
use serde::{
    de::{self, Error, MapAccess},
    Deserialize, Deserializer,
};
use std::{collections::HashMap, fmt};

mod common;
mod contents;
mod enumeration;

const KEY_ID: &str = "id";
const KEY_ORIG_ID: &str = "-orig-id";
const KEY_IF: &str = "if";
const KEY_DOC: &str = "doc";
const KEY_DOC_REF: &str = "doc-ref";

#[derive(Debug, PartialEq)]
pub enum AttributeType {
    Contents(Vec<u8>),
    Enumeration(enumeration::Enumeration),
}

#[derive(Debug, PartialEq)]
struct Attribute {
    id: String,
    doc: Option<String>,
    doc_ref: Option<String>,
    type_: AttributeType,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum StringOrU64<'a> {
    String(&'a str),
    U64(u64),
}

enum Size {
    Number(u64),
    Expression(String),
    EndOfStream(bool),
}

struct Enumeration(String);

struct TypeAttributes {
    terminator: Option<u8>,
}

#[derive(Default)]
struct Context {
    // string keys such as 'id', '-orig-id', 'if', etc
    string_keys: HashMap<&'static str, String>,
    size: Option<Size>,
    type_: Option<String>,
    type_attributes: Option<Either<contents::Bytes, Either<Enumeration, TypeAttributes>>>,
}

#[derive(Default)]
struct ContextNoContents {
    // string keys such as 'id', '-orig-id', 'if', etc
    string_keys: HashMap<&'static str, String>,
    size: Option<Size>,
    type_: Option<String>,
    type_attributes: Option<Either<Enumeration, TypeAttributes>>,
}

#[derive(Default)]
struct ContextTypeAttributes {
    // string keys such as 'id', '-orig-id', 'if', etc
    string_keys: HashMap<&'static str, String>,
    size: Option<Size>,
    type_: Option<String>,
    type_attributes: Option<TypeAttributes>,
}

fn build_attribute<'de, A: MapAccess<'de>>(context: Context) -> Result<Attribute, A::Error> {
    let context = match contents::try_build::<A>(context) {
        Either::Left(result) => return result,
        Either::Right(context) => context,
    };
    let context = match enumeration::try_build::<A>(context) {
        Either::Left(result) => return result,
        Either::Right(context) => context,
    };
    todo!()
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
                formatter.write_str("a struct with fields such 'id', 'doc', 'doc-ref'")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut context = Context::default();

                // helper macro to eliminate copy-paste
                macro_rules! parse_key {
                    ($key:ident, $key_next:ident) => {
                        match $key_next {
                            None => return build_attribute::<A>(context),
                            Some(key) if key == $key => {
                                context
                                    .string_keys
                                    .insert($key, map.next_value::<String>()?);

                                map.next_key::<&str>()?
                            }
                            maybe_key => maybe_key,
                        }
                    };
                }

                let key_next = map.next_key::<&str>()?;
                let key_next = parse_key!(KEY_ID, key_next);
                let key_next = parse_key!(KEY_ORIG_ID, key_next);

                // "size"/"size-eos"
                let key_next = match key_next {
                    None => return build_attribute::<A>(context),
                    Some(key) => {
                        if key == "size-eos" {
                            context.size = Some(Size::EndOfStream(map.next_value::<bool>()?));

                            map.next_key::<&str>()?
                        } else if key == "size" {
                            context.size = Some(match map.next_value::<StringOrU64>()? {
                                StringOrU64::String(expr) => Size::Expression(expr.to_string()),
                                StringOrU64::U64(number) => Size::Number(number),
                            });

                            map.next_key::<&str>()?
                        } else {
                            Some(key)
                        }
                    }
                };

                // "type". TODO: it maybe 'switch-on' expression
                let key_next = match key_next {
                    None => return build_attribute::<A>(context),
                    Some(key) if key == "type" => {
                        context.type_ = Some(map.next_value::<String>()?);

                        map.next_key::<&str>()?
                    }
                    maybe_key => maybe_key,
                };

                // "enum"/"contents" or other type (array) attributes
                let key_next = match key_next {
                    None => return build_attribute::<A>(context),
                    Some(key) => {
                        if key == "enum" {
                            let name = map.next_value::<String>()?;
                            context.type_attributes =
                                Some(Either::Right(Either::Left(Enumeration(name))));

                            map.next_key::<&str>()?
                        } else if key == "contents" {
                            context.type_attributes =
                                Some(Either::Left(map.next_value::<contents::Bytes>()?));

                            map.next_key::<&str>()?
                        } else {
                            // all other possible keys
                            Some(key)
                        }
                    }
                };

                let key_next = parse_key!(KEY_IF, key_next);
                let key_next = parse_key!(KEY_DOC, key_next);
                let key_next = parse_key!(KEY_DOC_REF, key_next);

                match key_next {
                    None => build_attribute::<A>(context),
                    Some(key) => Err(Error::custom(format!("unknown or duplicate field `{key}`"))),
                }
            }
        }

        deserializer.deserialize_map(AttributeVisitor)
    }
}
