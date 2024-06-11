#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum StringOrByte<'a> {
    String(&'a str),
    Byte(u8),
}

#[derive(Debug)]
pub struct ContentsBytes(pub Vec<u8>);


impl<'de> Deserialize<'de> for ContentsBytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct MyDataVisitor;

        impl<'de> serde::de::Visitor<'de> for MyDataVisitor {
            type Value = ContentsBytes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a object with fields 'id', 'type', and 'doc'")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::SeqAccess<'de>,
            {
                let mut bytes = Vec::with_capacity(1_000);
                while let Some(string_or_byte) = seq.next_element::<StringOrByte>()? {
                    match string_or_byte {
                        StringOrByte::Byte(b) => bytes.push(b),
                        StringOrByte::String(string) => {
                            bytes.extend_from_slice(string.as_bytes());
                        }
                    }
                }

                Ok(ContentsBytes(bytes))
            }
        }

        deserializer.deserialize_seq(MyDataVisitor)
    }
}
