use rtoml::prelude::*;
use rtoml::{TomlKey, TomlValue};
use std::convert::TryFrom;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::string::String;

fn main() -> Result<(), Box<dyn Error>> {
    let mut data = String::new();
    let mut file = File::open(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/test.toml"))?;

    file.read_to_string(&mut data)?;

    let toml = TomlValue::try_from(data.as_str())?;

    if let Some(table) = toml.as_table() {
        if let Some(key_value) = table.get(&TomlKey::from("a_table")) {
            assert_eq!(key.get(
                TomlKey::from("value"),
                TomlValue::Literal(String::from("hello, world"))
            ))
        }
    }

    println!("TOML\n\n{}\n", data);

    println!("PARSED\n{}\n", TomlValue::parse(&data).unwrap());

    Ok(())
}
