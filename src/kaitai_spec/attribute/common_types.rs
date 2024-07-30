#[derive(Debug, PartialEq)]
pub enum IntType {
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

pub fn type_parse<'de, A>(type_unchecked: String) -> Result<IntType, A::Error>
where
    A: serde::de::MapAccess<'de>,
{
    let endian: Option<Endian> = match type_unchecked.len() {
        4 => match &type_unchecked[2..4] {
            "le" => Some(Endian::Little),
            "be" => Some(Endian::Big),
            _ => return Err(serde::de::Error::custom("invalid type")),
        },
        2 => None,
        _ => return Err(serde::de::Error::custom("invalid type")),
    };

    let type_: IntType = match &type_unchecked[0..2] {
        "u1" => IntType::U1,
        "u2" => IntType::Long {
            type_: LongType::U2,
            endian,
        },
        "u4" => IntType::Long {
            type_: LongType::U4,
            endian,
        },
        "u8" => IntType::Long {
            type_: LongType::U8,
            endian,
        },
        "s1" => IntType::S1,
        "s2" => IntType::Long {
            type_: LongType::S2,
            endian,
        },
        "s4" => IntType::Long {
            type_: LongType::S4,
            endian,
        },
        "s8" => IntType::Long {
            type_: LongType::S8,
            endian,
        },
        _ => return Err(serde::de::Error::custom("invalid type")),
    };

    return Ok(type_);
}
