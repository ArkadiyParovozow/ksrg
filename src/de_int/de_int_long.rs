use serde::{Deserialize, Deserializer};
use crate::ENDIAN;
use serde_yaml;
use regex::{Captures, Regex};
use crate::de_int::de_int_data;

#[derive(Debug, PartialEq)]
struct IntegerLong {
    id:String,
    is_unsigned:bool,
    doc:Option<String>,
    endian:String,
    bytes:u64,
}

impl Default for IntegerLong{
    fn default() -> Self {
        Self{
            id : String::new(),
            is_unsigned:true,
            doc : None,
            bytes : 2,
            endian : String::from(ENDIAN),
        }
    }
}

impl<'de> Deserialize<'de> for IntegerLong {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct MyDataVisitor;

        impl<'de> serde::de::Visitor<'de> for MyDataVisitor {
            type Value = IntegerLong;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a object with fields 'id', 'type', and 'doc'")
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::MapAccess<'de>,
            {

                let (id, type_str, doc): (String, String, Option<String>) = de_int_data(map)?;

                let reg: Regex = Regex::new(r"([us])(\d+)(le|be)?$").unwrap();
                let captures: Captures = reg.captures(&type_str).expect("Invalid type format");
                let is_unsigned: bool = captures.get(1).map_or("u", |m| m.as_str()).eq("u");
                let bytes: u64 = captures.get(2).map_or("", |m| m.as_str()).parse().unwrap();
                let endian: String =  captures.get(3).map_or(IntegerLong::default().endian, |m| m.as_str().to_string());

                Ok(IntegerLong {
                    id,
                    is_unsigned,
                    endian,
                    bytes,
                    doc,
                })
            }
        }

        deserializer.deserialize_map(MyDataVisitor)
    }
}
 fn deserialize_int_long(input:&str) ->IntegerLong{
     return serde_yaml::from_str(input).expect("Failed to deserialize");
 }
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn simple_int_long(){
        let str: &str = r#"
        id: birth_year
        type: u16le
        doc: My doc"#;
        let _deserialized: IntegerLong = deserialize_int_long(str);
        let expect = IntegerLong{
            id:String::from("birth_year"),
            is_unsigned:true,
            doc:Some(String::from("My doc")),
            endian:String::from("le"),
            bytes:16,
        };
        assert_eq!(_deserialized,expect);
    }

    #[test]
    fn non_endian_int_long(){
        let str: &str = r#"
        id: birth_year
        type: s16"#;
        let _deserialized: IntegerLong = deserialize_int_long(str);
        let expect = IntegerLong{
            id:String::from("birth_year"),
            is_unsigned:false,
            doc: None,
            endian:String::from(ENDIAN),
            bytes:16,
        };
        assert_eq!(_deserialized,expect);
    }

    #[test]
    fn non_doc_value_int_long(){
        let str: &str = r#"
        id: birth_year
        type: u16le"#;
        let _deserialized: IntegerLong = deserialize_int_long(str);
        let expect = IntegerLong{
            id:String::from("birth_year"),
            is_unsigned:true,
            doc: None,
            endian:String::from("le"),
            bytes:16,
        };
        assert_eq!(_deserialized,expect);
    }

    #[test]
    #[should_panic]
    fn non_id_int_long(){
        let str: &str = r#"
        type: u16le"#;
        let _deserialized: IntegerLong = deserialize_int_long(str);
    }

    #[test]
    #[should_panic]
    fn duplicate_fields_int_long(){
        let str: &str = r#"
        id: birth_year
        id: madagaskar
        type: u16le"#;
        let _deserialized: IntegerLong = deserialize_int_long(str);
    }

    #[test]
    #[should_panic]
    fn incorrect_type1_int_long(){
        let str: &str = r#"
        id: birth_year
        type: g16le"#;
        let _deserialized: IntegerLong = deserialize_int_long(str);
    }

    #[test]
    #[should_panic]
    fn incorrect_type2_int_long(){
        let str: &str = r#"
        id: birth_year
        type: s16lele"#;
        let _deserialized: IntegerLong = deserialize_int_long(str);
    }

    #[test]
    #[should_panic]
    fn incorrect_fields_int_long(){
        let str: &str = r#"
        id: birth_year
        type: u1
        Hype: hehe"#;
        let _deserialized: IntegerLong = deserialize_int_long(str);
    }
}
