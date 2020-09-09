use rtoml::ast::Node::Value;
use rtoml::ast::TomlValue::String;
use rtoml::parser::RToml;

fn main() -> std::io::Result<()> {
    let toml = RToml::file("templates/test.toml");
    let reader = toml.parse()?;
    let val = reader.get_table("some_table");
    if let Value(String(x)) = val.get("value").unwrap() {
        println!("{:?}", x);
    }
    Ok(())
}
