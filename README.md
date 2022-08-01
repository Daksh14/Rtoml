# Rtoml

Rtoml is a toml parser that's designed to

1. Acquire values at compile time to a struct.
2. Acquire values at runtime required to work. 

At the core, a Toml value is represented by this enum
```rust
pub enum TomlValue<'a> {
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<TomlValue<'a>>),
    Boolean(bool),
    DateTime(DateTime),
    Table(Table<'a>),
}
```

# Usage 

For a toml file
```toml
[a_table]
value = "hello, world"
```
We will use the following code to acquire values
```rust
use rtoml::prelude::*;
use std::error::Error;

fn main() -> Result<(), TomlError> {
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

    Ok(())
}
```

# Benchmarks
```
cargo bench
open ./target/criterion/report/index.html
```
Testing shows Rtoml is slightly faster.

# TODO
- [ ] Table dot `.` format in table names
- [ ] Table arrays
- [ ] Serialise derive traits
- [ ] Use toml testing repo to run all tests 

# License
MIT
