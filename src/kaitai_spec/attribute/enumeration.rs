#[derive(Debug, PartialEq)]
pub struct Enumeration {
    pub name: String,
    pub type_: EnumType,
}

#[derive(Debug, PartialEq)]
pub enum EnumType {
    U1,
    S1,
    Long {
        type_: LongType,
        endian: Option<Endian>,
    },
}

#[derive(Debug, PartialEq)]
pub enum LongType {
    U2,
    U4,
    U8,
    S2,
    S4,
    S8,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Endian {
    Little,
    Big,
}

pub fn process<'de, A>(
    mut common_keys: super::CommonKeys,
    mut map: A,
) -> Result<super::Attribute, A::Error>
where
    A: serde::de::MapAccess<'de>,
{
    let name: String = map.next_value::<String>()?;

    while let Some(key) = map.next_key::<&str>()? {
        if common_keys.process(key, &mut map)? {
            continue;
        }
        return Err(serde::de::Error::unknown_field(key, &["nothing expected"]));
    }
    let type_unchecked: String = common_keys
        .type_uncheck
        .ok_or_else(|| serde::de::Error::missing_field("type"))?;

    let endian: Option<Endian> = match type_unchecked.len() {
        4 => match &type_unchecked[2..4] {
            "le" => Some(Endian::Little),
            "be" => Some(Endian::Big),
            _ => return Err(serde::de::Error::custom("invalid type")),
        },
        2 => None,
        _ => return Err(serde::de::Error::custom("invalid type")),
    };

    let type_ = super::AttributeType::Enumeration(Enumeration {
        name,
        type_: match &type_unchecked[0..2] {
            "u1" => EnumType::U1,
            "u2" => EnumType::Long {
                type_: LongType::U2,
                endian,
            },
            "u4" => EnumType::Long {
                type_: LongType::U4,
                endian,
            },
            "u8" => EnumType::Long {
                type_: LongType::U8,
                endian,
            },
            "s1" => EnumType::S1,
            "s2" => EnumType::Long {
                type_: LongType::S2,
                endian,
            },
            "s4" => EnumType::Long {
                type_: LongType::S4,
                endian,
            },
            "s8" => EnumType::Long {
                type_: LongType::S8,
                endian,
            },
            _ => return Err(serde::de::Error::custom("invalid type")),
        },
    });

    Ok(super::Attribute {
        id: common_keys
            .id
            .ok_or_else(|| serde::de::Error::missing_field("id"))?,
        doc: common_keys.doc,
        doc_ref: common_keys.doc_ref,
        type_,
    })
}
