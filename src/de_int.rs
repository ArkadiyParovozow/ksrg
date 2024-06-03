use serde::de::Error;

mod de_int_byte;
mod de_int_long;

fn de_int_data<'de,A>(mut map: A)-> Result<(String,String, Option<String>), A::Error>
    where
        A: serde::de::MapAccess<'de>,
{
    let mut id = None;
    let mut type_str = None;
    let mut doc = None;

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
            "doc" => {
                if doc.is_some() {
                    return Err(A::Error::duplicate_field("doc"));
                }
                doc = Some(map.next_value::<String>()?);
            }
            _ => {
                return Err(A::Error::unknown_field(&key, &["id","doc","type"]));
            }
        }
    }

    let id: String = id.ok_or_else(|| A::Error::missing_field("id"))?;
    let type_str: String = type_str.ok_or_else(|| A::Error::missing_field("type"))?;
    return Ok((id, type_str, doc))
}