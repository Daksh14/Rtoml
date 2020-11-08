use rtoml::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let toml = RToml::file(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/test.toml"));
    let reader = toml.parse()?;
    let val = reader.get_table_array("table_arr");
    println!("{:?}", val);

    Ok(())
}
