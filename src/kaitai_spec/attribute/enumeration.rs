use crate::kaitai_spec::attribute::{Attribute, AttributeType, CommonKeys};
use serde::de::Error;

const ENDIAN: Endian = Endian::Big;
#[derive(Debug, PartialEq)]
pub struct Enumeration {
    name: String,
    type_: EnumType,
}
#[derive(Debug, PartialEq)]
enum EnumType {
    U1,
    S1,
    Long { type_: LongType, endian: Endian },
}
#[derive(Debug, PartialEq)]
enum LongType {
    U2,
    U4,
    U8,
    S2,
    S4,
    S8,
}
#[derive(Debug, PartialEq, Clone)]
enum Endian {
    Little,
    Big,
}
pub fn process<'de, A>(
    name: String,
    mut common_keys: CommonKeys,
    mut map: A,
) -> Result<Attribute, A::Error>
where
    A: serde::de::MapAccess<'de>,
{
    let mut endian: Endian = ENDIAN; //DEFAULT
    let type_: Option<EnumType>;

    while let Some(key) = map.next_key::<&str>()? {
        if common_keys.process(key, &mut map)? {
            continue;
        }
        match key {
            _ => return Err(A::Error::unknown_field(key, &["nothing expected"])),
        }
    }
    let type_unchecked: String = common_keys
        .type_uncheck
        .ok_or_else(|| A::Error::missing_field("type"))?;

    match type_unchecked.chars().count() {
        4 => {
            let endian_unchecked: &str = &type_unchecked[2..4];
            match endian_unchecked {
                "le" => endian = Endian::Little,
                "be" => endian = Endian::Big,
                "" => {}
                _ => return Err(A::Error::custom("invalid type")),
            }
        }
        2 => {}
        _ => return Err(A::Error::custom("invalid type")),
    }

    type_ = match &type_unchecked[0..2] {
        "u1" => Some(EnumType::U1),
        "u2" => Some(EnumType::Long {
            type_: LongType::U2,
            endian,
        }),
        "u4" => Some(EnumType::Long {
            type_: LongType::U4,
            endian,
        }),
        "u8" => Some(EnumType::Long {
            type_: LongType::U8,
            endian,
        }),
        "s1" => Some(EnumType::S1),
        "s2" => Some(EnumType::Long {
            type_: LongType::S2,
            endian,
        }),
        "s4" => Some(EnumType::Long {
            type_: LongType::S4,
            endian,
        }),
        "s8" => Some(EnumType::Long {
            type_: LongType::S8,
            endian,
        }),
        _ => return Err(A::Error::custom("invalid type")),
    };
    let type_ = type_.ok_or_else(|| A::Error::custom("invalid type"))?;
    let id: String = common_keys
        .id
        .ok_or_else(|| A::Error::missing_field("id"))?;
    return Ok(Attribute {
        id,
        doc: common_keys.doc,
        doc_ref: common_keys.doc_ref,
        type_: AttributeType::Enumeration(Enumeration { name, type_ }),
    });
}

#[cfg(test)]
mod tests {

    use crate::kaitai_spec::attribute::AttributeType::Enumeration;
    use crate::kaitai_spec::attribute::{
        enumeration,
        enumeration::{Endian, EnumType, LongType},
        Attribute,
    };

    #[test]
    fn simple_enum1() {
        let str: &str = r#"
        id: hehe
        type: u2le
        enum: ip_prot
        doc: My doc"#;
        let _deserialized: Attribute = serde_yaml::from_str(str).expect("Failed!");
        let expect = Attribute {
            id: String::from("hehe"),
            doc: Some(String::from("My doc")),
            type_: Enumeration(enumeration::Enumeration {
                name: String::from("ip_prot"),
                type_: EnumType::Long {
                    type_: LongType::U2,
                    endian: Endian::Little,
                },
            }),
        };
        assert_eq!(_deserialized, expect);
    }

    #[test]
    fn simple_enum2() {
        let str: &str = r#"
        id: hehe
        type: s1
        enum: ip_prot
        doc: My doc"#;
        let _deserialized: Attribute = serde_yaml::from_str(str).expect("Failed!");
        let expect = Attribute {
            id: String::from("hehe"),
            doc: Some(String::from("My doc")),
            type_: Enumeration(enumeration::Enumeration {
                name: String::from("ip_prot"),
                type_: EnumType::S1,
            }),
        };
        assert_eq!(_deserialized, expect);
    }
    #[test]
    #[should_panic]
    fn non_type_enum() {
        let str: &str = r#"
        id: birth_year
        enum: ugabuga
        doc: My doc"#;
        let _deserialized: Attribute = serde_yaml::from_str(str).expect("Failed!");
    }
    #[test]
    #[should_panic]
    fn incorrect_type_enum1() {
        let str: &str = r#"
        id: birth_year
        enum: ugabuga
        type: uof:abg;a
        doc: My doc"#;
        let _deserialized: Attribute = serde_yaml::from_str(str).expect("Failed!");
    }

    #[test]
    #[should_panic]
    fn incorrect_type_enum2() {
        let str: &str = r#"
        id: birth_year
        enum: ugabuga
        type: u2lee
        doc: My doc"#;
        let _deserialized: Attribute = serde_yaml::from_str(str).expect("Failed!");
    }
}
