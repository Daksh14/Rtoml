use std::convert::TryFrom;
use std::error::Error;
use std::io::{stdin, BufRead, Read};
use std::string::String;

use rtoml::prelude::*;
use serde_json::Value;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer)?;

    println!(
        "{}",
        to_json_value(TomlValue::try_from(buffer.as_str()).expect("Parsing error"))
    );

    Ok(())
}

fn to_json_value(value: TomlValue) -> Value {
    let mut value = match value {
        TomlValue::Int(x) => Value::from(x),
        TomlValue::Float(x) => Value::from(x),
        TomlValue::Boolean(x) => Value::from(x),
        TomlValue::String(x) => Value::from(x),
        TomlValue::DateTime(x) => Value::from(x.to_string()),
        TomlValue::Array(x) => x.into_iter().map(to_json_value).collect(),
        TomlValue::Table(x) => x
            .into_iter()
            .map(|(key, value)| (key.to_string(), to_json_value(value)))
            .collect(),
    };

    if let Some(object) = value.as_object_mut() {
        if let Some(empty) = object.remove("") {
            value = empty
        }
    }

    value
}
