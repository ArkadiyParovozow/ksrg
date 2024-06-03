use regex::{Captures, Regex};
use serde::{Deserialize, Deserializer};
use serde_yaml;
use crate::de_int::de_int_data;

#[derive(Debug, PartialEq)]
struct IntegerByte{
    id:String,
    is_unsigned:bool,
    doc:Option<String>,
}

impl Default for IntegerByte{
    fn default() -> Self {
        Self{
            id : String::new(),
            doc : None,
            is_unsigned : true,
        }
    }
}
impl<'de> Deserialize<'de> for IntegerByte {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct MyDataVisitor;

        impl<'de> serde::de::Visitor<'de> for MyDataVisitor {
            type Value = IntegerByte;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a object with fields 'id', 'type', and 'doc'")
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::MapAccess<'de>,
            {
                let (id, type_str, doc): (String, String, Option<String>) = de_int_data(map)?;

                let reg: Regex = Regex::new(r"([us])(1)$").unwrap();
                let captures: Captures = reg.captures(&type_str).expect("Invalid type format");
                let is_unsigned: bool = captures.get(1).map_or("u", |m| m.as_str()).eq("u");

                Ok(IntegerByte {
                    id,
                    is_unsigned,
                    doc,
                })
            }
        }

        deserializer.deserialize_map(MyDataVisitor)
    }
}
fn deserialize_int_long(input:&str) -> IntegerByte {
    return serde_yaml::from_str(input).expect("Failed to deserialize");
}


 fn deserialize_int_byte(input:&str) ->IntegerByte{
     return serde_yaml::from_str(input).expect("Failed to deserialize");
 }

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn simple_int_byte(){
        let str: &str = r#"
        id: birth_year
        type: u1
        doc: My doc"#;
        let _deserialized: IntegerByte = deserialize_int_byte(str);
        let expect = IntegerByte{
            id:String::from("birth_year"),
            is_unsigned:true,
            doc:Some(String::from("My doc")),
        };
        assert_eq!(_deserialized,expect);
    }

    #[test]
    fn non_doc_int_byte(){
        let str: &str = r#"
        id: birth_year
        type: s1"#;
        let _deserialized: IntegerByte = deserialize_int_byte(str);
        let expect = IntegerByte{
            id:String::from("birth_year"),
            is_unsigned:false,
            doc: None,
        };
        assert_eq!(_deserialized,expect);
    }

    #[test]
    #[should_panic]
    fn non_id_int_byte(){
        let str: &str = r#"
        type: s1"#;
        let _deserialized: IntegerByte = deserialize_int_byte(str);
    }

    #[test]
    #[should_panic]
    fn endian_for_int_byte(){
        let str: &str = r#"
        id: birth_year
        type: u2be"#;
        let _deserialized: IntegerByte = deserialize_int_byte(str);
    }

    #[test]
    #[should_panic]
    fn incorrect_type1_int_byte(){
        let str: &str = r#"
        id: birth_year
        type: u2"#;
        let _deserialized: IntegerByte = deserialize_int_byte(str);
    }

    #[test]
    #[should_panic]
    fn incorrect_type2_byte(){
        let str: &str = r#"
        id: birth_year
        type: g2"#;
        let _deserialized: IntegerByte = deserialize_int_byte(str);
    }

}