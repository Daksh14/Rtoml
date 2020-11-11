# Rtoml

Rtoml is a toml parser that's designed to

1. Acquire values at compile time to a struct.
2. Acquire values at runtime required to work. 

At the core, a Toml value is represented by this enum
```rust
pub enum TomlValue {
    Int(i64),
    Floating(f64),
    String(String),
    Array(Vec<TomlValue>),
    Boolean(bool),
    InlineTable(HashMap<String, TomlValue>),
}
```
An Inline table is a hashmap and an array is a vector of `TomlValue`. 

# Usage 

For a toml file
```toml
[table]
value = "hello, world"
```
We will use the following code to aquire values
```rust
use rtoml::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let toml = RToml::file(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/test.toml"));
    let reader = toml.parse()?;
    let val = reader.get_table("table");
    assert!("hello, world".to_string() == val.get("value").unwrap().get_string().unwrap());

    Ok(())
}
```

# TODO
- [ ] Errors on specific line numbers
- [ ] + or - number prefixes
- [ ] Table dot `.` format in table names
- [ ] Validity testing
