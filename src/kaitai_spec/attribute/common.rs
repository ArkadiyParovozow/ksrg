use serde::de::{Error, MapAccess};

#[derive(Debug, PartialEq)]
pub enum Integer {
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

pub fn type_parse<'de, A>(type_unchecked: &str) -> Result<Integer, A::Error>
where
    A: MapAccess<'de>,
{
    let endian: Option<Endian> = match type_unchecked.len() {
        4 => match &type_unchecked[2..4] {
            "le" => Some(Endian::Little),
            "be" => Some(Endian::Big),
            _ => return Err(Error::custom("invalid type")),
        },
        2 => None,
        _ => return Err(Error::custom("invalid type")),
    };

    let type_: Integer = match &type_unchecked[0..2] {
        "u1" => Integer::U1,
        "u2" => Integer::Long {
            type_: LongType::U2,
            endian,
        },
        "u4" => Integer::Long {
            type_: LongType::U4,
            endian,
        },
        "u8" => Integer::Long {
            type_: LongType::U8,
            endian,
        },
        "s1" => Integer::S1,
        "s2" => Integer::Long {
            type_: LongType::S2,
            endian,
        },
        "s4" => Integer::Long {
            type_: LongType::S4,
            endian,
        },
        "s8" => Integer::Long {
            type_: LongType::S8,
            endian,
        },
        _ => return Err(Error::custom("invalid type")),
    };

    return Ok(type_);
}
