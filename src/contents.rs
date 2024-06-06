use serde::{de, Deserialize, Deserializer,};
use serde_yaml;
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Contents
{
    pub id: Option<String>,
    #[serde(deserialize_with = "deserialize_contents")]
    pub contents: Vec<u8>,
}

fn deserialize_contents<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    struct ContentsVisitor;

    impl<'de> de::Visitor<'de> for ContentsVisitor {
        type Value = Vec<u8>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("string or byte array")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.as_bytes().to_owned())
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.into_bytes())
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            let mut vec = Vec::with_capacity(seq.size_hint().unwrap_or(0));
            while let Some(elem) = seq.next_element::<serde_yaml::Value>()? {
                match elem {
                    serde_yaml::Value::String(s) => {
                        // Interpreting the string as bytes
                        let bytes = if s.starts_with("0x") {
                            s.as_bytes().to_owned()
                        } else {
                            s.into_bytes()
                        };
                        vec.extend(bytes);
                    }
                    serde_yaml::Value::Number(n) => {
                        if let Some(byte) = n.as_u64() {
                            vec.push(byte as u8);
                        } else {
                            return Err(de::Error::custom("Invalid number"));
                        }
                    }
                    _ => return Err(de::Error::custom("Invalid type")),
                }
            }
            Ok(vec)
        }
    }

    // The function is only called if contents variable is non null.
    Ok(deserializer.deserialize_any(ContentsVisitor)?)
}
#[cfg(test)]
mod tests {
    use super::*;
#[test]
    fn simple_de() {
        let yaml = "
      id: animal_record
      contents: [0xca, 0xfe, 0xba, 0xbe]
    ";

    let deserialized_content = serde_yaml::from_str::<Contents>(&yaml);
    assert!(
        matches!(deserialized_content, Ok(content) if content.contents == vec![0xCA, 0xFE, 0xBA, 0xBE])
    );
    }
#[test]
    fn without_id() {
        let yaml = "
      contents: [0xca, 0xfe, 0xba, 0xbe]
    ";

    let deserialized_content = serde_yaml::from_str::<Contents>(&yaml);
    assert!(
        matches!(deserialized_content, Ok(content) if content.contents == vec![0xCA, 0xFE, 0xBA, 0xBE])
    );
    }
#[test]
    fn string_case() {
        let yaml = "
        id: magic1
        contents: JFIF
    ";

    let deserialized_content = serde_yaml::from_str::<Contents>(&yaml);
    assert!(
        matches!(deserialized_content, Ok(content) if content.contents == vec![74, 70, 73, 70])
    );
    }
#[test]
    fn extra_field() {
        let yaml = "
        id: magic1
        contents: JFIF
        doc: AFsp metadata
    ";

    let deserialized_content = serde_yaml::from_str::<Contents>(&yaml);
    assert!(!deserialized_content.is_ok());
    }
#[test]
    fn string_and_number_case() {
        let yaml = "
        id: magic3
        contents: [CAFE, 0, BABE]
    ";

    let deserialized_content = serde_yaml::from_str::<Contents>(&yaml);
    assert!(
        matches!(deserialized_content, Ok(content) if content.contents == vec![67, 65, 70, 69, 0, 66, 65, 66, 69])
    );
    }
#[test]
    fn byte_and_number_case() {
        let yaml = "
        id: magic4
        contents: [foo, 0, A, 0xa, 42]
    ";

    let deserialized_content = serde_yaml::from_str::<Contents>(&yaml);
    assert!(
        matches!(deserialized_content, Ok(content) if content.contents == vec![102, 111, 111, 0, 65, 10, 42])
    );
    }
#[test]
    fn extreme_example() {
        let yaml = "
        id: magic5
        contents: [1, 0x55, '▒,3', 3]
    ";

    let deserialized_content = serde_yaml::from_str::<Contents>(&yaml);
    assert!(
        matches!(deserialized_content, Ok(content) if content.contents == vec![1, 85, 226, 150, 146, 44, 51, 3])
    );
    }
}