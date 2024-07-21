use super::common_types::IntType;

#[derive(Debug, PartialEq)]
pub struct Enumeration {
    pub name: String,
    pub type_: IntType,
}

pub fn process<'de, A>(
    mut common_keys: super::CommonKeys,
    mut map: A,
) -> Result<super::Attribute, A::Error>
where
    A: serde::de::MapAccess<'de>,
{
    let name: String = map.next_value::<String>()?;

    while let Some(key) = map.next_key::<&str>()? {
        if common_keys.process(key, &mut map)? {
            continue;
        }
        return Err(serde::de::Error::unknown_field(key, &["nothing expected"]));
    }
    let type_unchecked: String = common_keys
        .type_
        .ok_or_else(|| serde::de::Error::missing_field("type"))?;

    let type_: IntType = super::common_types::type_parse::<A>(type_unchecked)?;

    Ok(super::Attribute {
        id: common_keys
            .id
            .ok_or_else(|| serde::de::Error::missing_field("id"))?,
        doc: common_keys.doc,
        doc_ref: common_keys.doc_ref,
        type_: super::AttributeType::Enumeration(Enumeration { name, type_ }),
    })
}
