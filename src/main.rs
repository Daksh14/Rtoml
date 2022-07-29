use rtoml::prelude::*;
use rtoml::TomlValue;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::string::String;

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("templates/test.toml").expect("Error opening file");
    let mut data = String::new();

    file.read_to_string(&mut data);

    println!("TOML\n\n{}\n", data);

    println!("PARSED\n{}\n", TomlValue::parse(&data).unwrap());

    Ok(())
}
