use super::*;
use crate::kaitai_spec;

#[test]
fn simple_de() {
    let yaml = "
            id: magic1
            doc: example
            doc-ref: ref1
            contents: [0xca, 0xfe, 0xba, 0xbe]
        ";

    let attribute: Attribute = serde_yaml::from_str(yaml).unwrap();

    let Attribute {
        id,
        doc,
        doc_ref,
        type_,
    } = attribute;

    assert_eq!(id, "magic1");
    assert_eq!(doc, Some("example".to_string()));
    assert_eq!(doc_ref, Some("ref1".to_string()));
    let contents = match type_ {
        AttributeType::Contents(contents) => contents,
        _ => unreachable!(),
    };

    assert_eq!(contents, vec![0xCA, 0xFE, 0xBA, 0xBE]);
}

#[test]
fn without_id() {
    let yaml = "
            doc: example
            doc-ref: ref1
            contents: JFIF
        ";
    let attribute: Attribute = serde_yaml::from_str(yaml).unwrap();
    let Attribute {
        id,
        doc,
        doc_ref,
        type_,
    } = attribute;

    assert_eq!(id, "");
    assert_eq!(doc, Some("example".to_string()));
    assert_eq!(doc_ref, Some("ref1".to_string()));
    let contents = match type_ {
        AttributeType::Contents(contents) => contents,
        _ => unreachable!(),
    };

    assert_eq!(contents, b"JFIF".to_vec());
}

#[test]
fn string_case() {
    let yaml = "
        id: magic1
        doc: example
        doc-ref: ref1
        contents: JFIF
    ";

    let attribute: Attribute = serde_yaml::from_str(yaml).unwrap();

    let Attribute {
        id,
        doc,
        doc_ref,
        type_,
    } = attribute;

    assert_eq!(id, "magic1");
    assert_eq!(doc, Some("example".to_string()));
    assert_eq!(doc_ref, Some("ref1".to_string()));
    let contents = match type_ {
        AttributeType::Contents(contents) => contents,
        _ => unreachable!(),
    };

    assert_eq!(contents, b"JFIF".to_vec());
}

#[test]
fn extra_field() {
    let yaml = "
        id: magic1
        doc: example
        doc-ref: ref1
        contents: JFIF
        extra_field:
    ";

    let result: Result<Attribute, _> = serde_yaml::from_str(yaml);

    assert!(result.is_err());
}

#[test]
fn string_and_number_case() {
    let yaml = "
        id: magic1
        doc: example
        doc-ref: ref1
        contents: [CAFE, 0, BABE]
    ";

    let attribute: Attribute = serde_yaml::from_str(yaml).unwrap();

    let Attribute {
        id,
        doc,
        doc_ref,
        type_,
    } = attribute;

    assert_eq!(id, "magic1");
    assert_eq!(doc, Some("example".to_string()));
    assert_eq!(doc_ref, Some("ref1".to_string()));
    let contents = match type_ {
        AttributeType::Contents(contents) => contents,
        _ => unreachable!(),
    };

    assert_eq!(contents, vec![67, 65, 70, 69, 0, 66, 65, 66, 69]);
}

#[test]
fn byte_and_number_case() {
    let yaml = "
        id: magic1
        doc: example
        doc-ref: ref1
        contents: [foo, 0, A, 0xa, 42]
    ";

    let attribute: Attribute = serde_yaml::from_str(yaml).unwrap();

    let Attribute {
        id,
        doc,
        doc_ref,
        type_,
    } = attribute;

    assert_eq!(id, "magic1");
    assert_eq!(doc, Some("example".to_string()));
    assert_eq!(doc_ref, Some("ref1".to_string()));
    let contents = match type_ {
        AttributeType::Contents(contents) => contents,
        _ => unreachable!(),
    };

    assert_eq!(contents, vec![102, 111, 111, 0, 65, 10, 42]);
}

#[test]
fn extreme_example() {
    let yaml = "
        id: magic1
        doc: example
        doc-ref: ref1
        contents: [1, 0x55, '▒,3', 3]
    ";

    let attribute: Attribute = serde_yaml::from_str(yaml).unwrap();

    let Attribute {
        id,
        doc,
        doc_ref,
        type_,
    } = attribute;

    assert_eq!(id, "magic1");
    assert_eq!(doc, Some("example".to_string()));
    assert_eq!(doc_ref, Some("ref1".to_string()));
    let contents = match type_ {
        AttributeType::Contents(contents) => contents,
        _ => unreachable!(),
    };

    assert_eq!(contents, vec![1, 85, 226, 150, 146, 44, 51, 3]);
}

#[test]
fn duplicate_contents() {
    let yaml = "
            id: magic1
            doc: example
            doc-ref: ref1
            contents: JFIF
            contents: JFIF
        ";

    let result: Result<Attribute, _> = serde_yaml::from_str(yaml);

    assert!(result.is_err());
}

#[test]
fn simple_enum1() {
    let str: &str = r#"
        id: id_1
        type: u2le
        enum: ip_prot
        doc: My doc"#;
    let _deserialized: Attribute = serde_yaml::from_str(str).expect("Failed!");
    let expect = Attribute {
        id: String::from("id_1"),
        doc_ref: None,
        doc: Some(String::from("My doc")),
        type_: AttributeType::Enumeration(enumeration::Enumeration {
            name: String::from("ip_prot"),
            type_: common_types::IntType::Long {
                type_: common_types::LongType::U2,
                endian: Some(common_types::Endian::Little),
            },
        }),
    };
    assert_eq!(_deserialized, expect);
}

#[test]
fn simple_enum2() {
    let str: &str = r#"
        id: data1
        type: s1
        enum: ip_prot
        doc: My doc"#;
    let _deserialized: Attribute = serde_yaml::from_str(str).expect("Failed!");
    let expect = Attribute {
        id: String::from("data1"),
        doc: Some(String::from("My doc")),
        doc_ref: None,
        type_: AttributeType::Enumeration(enumeration::Enumeration {
            name: String::from("ip_prot"),
            type_: common_types::IntType::S1,
        }),
    };
    assert_eq!(_deserialized, expect);
}

#[test]
#[should_panic]
fn incorrect_type_enum1() {
    let str: &str = r#"
        id: birth_year
        enum: MA1
        type: incorrect
        doc: My doc"#;
    let _deserialized: Attribute = serde_yaml::from_str(str).expect("Failed!");
}

#[test]
#[should_panic]
fn incorrect_type_enum2() {
    let str: &str = r#"
        id: birth_year
        enum: MA1
        type: u2lee
        doc: My doc"#;
    let _deserialized: Attribute = serde_yaml::from_str(str).expect("Failed!");
}

#[test]
fn simple_int1() {
    let str: &str = r#"
        id: id_1
        type: u2le
        doc: My doc"#;
    let _deserialized: Attribute = serde_yaml::from_str(str).expect("Failed!");
    let expect = Attribute {
        id: String::from("id_1"),
        doc_ref: None,
        doc: Some(String::from("My doc")),
        type_: AttributeType::Integer(common_types::IntType::Long {
            type_: common_types::LongType::U2,
            endian: Some(common_types::Endian::Little),
        }),
    };
    assert_eq!(_deserialized, expect);
}

#[test]
fn simple_int2() {
    let str: &str = r#"
        id: data1
        type: s1
        doc: My doc"#;
    let _deserialized: Attribute = serde_yaml::from_str(str).expect("Failed!");
    let expect = Attribute {
        id: String::from("data1"),
        doc_ref: None,
        doc: Some(String::from("My doc")),
        type_: AttributeType::Integer(common_types::IntType::S1),
    };
    assert_eq!(_deserialized, expect);
}

#[test]
#[should_panic]
fn incorrect_type_int1() {
    let str: &str = r#"
        id: birth_year
        type: incorrect
        doc: My doc"#;
    let _deserialized: Attribute = serde_yaml::from_str(str).expect("Failed!");
}

#[test]
#[should_panic]
fn incorrect_type_int2() {
    let str: &str = r#"
        id: birth_year
        type: u2lee
        doc: My doc"#;
    let _deserialized: Attribute = serde_yaml::from_str(str).expect("Failed!");
}
