use rtoml::prelude::*;
use std::convert::TryFrom;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::string::String;

fn main() -> Result<(), Box<dyn Error>> {
    let mut data = String::new();
    let mut file = File::open(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/test.toml"))?;

    file.read_to_string(&mut data)?;

    let toml = TomlValue::try_from(data.as_str()).unwrap();

    println!("{:#?}", toml);

    Ok(())
}
