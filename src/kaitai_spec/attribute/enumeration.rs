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
