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

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
            while let Some(i) = seq.next_element()? {
                vec.push(i);
            }
            Ok(vec)
        }
    }

    // The function is only called if contents variable is non null.
    Ok(deserializer.deserialize_any(ContentsVisitor)?)
}
#[test]
    fn simple_de() {
        let yaml = "
      id: animal_record
      contents: [0xca, 0xfe, 0xba, 0xbe]
    ";

    let deserialized_content = serde_yaml::from_str::<Contents>(&yaml);
    assert!(deserialized_content.is_ok());
    }
#[test]
    fn without_id() {
        let yaml = "
      contents: [0xca, 0xfe, 0xba, 0xbe]
    ";

    let deserialized_content = serde_yaml::from_str::<Contents>(&yaml);
    assert!(deserialized_content.is_ok());
    }
#[test]
    fn string_case() {
        let yaml = "
        id: magic1
        contents: JFIF
    ";

    let deserialized_content = serde_yaml::from_str::<Contents>(&yaml);
    assert!(deserialized_content.is_ok());
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