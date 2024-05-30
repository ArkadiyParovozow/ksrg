use regex::{Captures, Regex};
use serde::{de::Error, Deserialize, Deserializer};
use serde_yaml;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct IntegerByte{
    id:String,
    is_unsigned:bool,
    size:u64,
}

impl Default for IntegerByte{
    fn default() -> Self {
        Self{
            id : String::new(),
            size : 1,
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
                formatter.write_str("a object with fields 'id', 'type', and 'size'")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::MapAccess<'de>,
            {
                let mut id = None;
                let mut type_str = None;
                let mut size = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "id" => {
                            if id.is_some() {
                                return Err(A::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value::<String>()?);
                        }
                        "type" => {
                            if type_str.is_some() {
                                return Err(A::Error::duplicate_field("type"));
                            }
                            type_str = Some(map.next_value::<String>()?);
                        }
                        "size" => {
                            if size.is_some() {
                                return Err(A::Error::duplicate_field("size"));
                            }
                            size = Some(map.next_value::<u64>()?);
                        }
                        _ => {
                            return Err(A::Error::unknown_field(&key, &["id","size","type"]));
                        }
                    }
                }

                let id: String = id.ok_or_else(|| A::Error::missing_field("id"))?;
                let type_str: String = type_str.ok_or_else(|| A::Error::missing_field("type"))?;
                let size: u64 = size.unwrap_or(
                    IntegerByte::default().size
                );

                let reg: Regex = Regex::new(r"([us])(1)$").unwrap();
                let captures: Captures = reg.captures(&type_str).expect("Invalid type format");
                let is_unsigned: bool = captures.get(1).map_or("u", |m| m.as_str()).eq("u");

                Ok(IntegerByte {
                    id,
                    is_unsigned,
                    size,
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
        size: 20"#;
        let deserialized: IntegerByte = deserialize_int_byte(str);
        let expect = IntegerByte{
            id:String::from("birth_year"),
            is_unsigned:true,
            size:20,
        };
        assert_eq!(deserialized,expect);
    }

    #[test]
    fn non_size_int_byte(){
        let str: &str = r#"
        id: birth_year
        type: s1"#;
        let deserialized: IntegerByte = deserialize_int_byte(str);
        let expect = IntegerByte{
            id:String::from("birth_year"),
            is_unsigned:false,
            size:1,
        };
        assert_eq!(deserialized,expect);
    }

    #[test]
    #[should_panic]
    fn non_id_int_byte(){
        let str: &str = r#"
        type: s1"#;
        let deserialized: IntegerByte = deserialize_int_byte(str);
    }

    #[test]
    #[should_panic]
    fn endian_for_int_byte(){
        let str: &str = r#"
        id: birth_year
        type: u2be"#;
        let deserialized: IntegerByte = deserialize_int_byte(str);
    }

    #[test]
    #[should_panic]
    fn incorrect_type1_int_byte(){
        let str: &str = r#"
        id: birth_year
        type: u2"#;
        let deserialized: IntegerByte = deserialize_int_byte(str);
    }

    #[test]
    #[should_panic]
    fn incorrect_type2_byte(){
        let str: &str = r#"
        id: birth_year
        type: g2"#;
        let deserialized: IntegerByte = deserialize_int_byte(str);
    }

}