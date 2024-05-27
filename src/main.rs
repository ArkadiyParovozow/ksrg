use serde::{Serialize, Deserialize};
use serde_yaml;
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Contents
{
    pub id: String,
    pub contents: Vec<u8>,
}

fn main() -> Result<(), serde_yaml::Error> {
    let yaml = "
      id: animal_record
      contents:
      - 0xca
      - 0xfe
      - 0xba
      - 0xbe
    ";

    let deserialized_content = serde_yaml::from_str::<Contents>(&yaml)?;
    println!("{:?}", deserialized_content);
    Ok(())
}