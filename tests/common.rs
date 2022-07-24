use serde_json::{Result, Value};
use std::fs;

pub fn get_all_files(dir: &'static str) -> (Vec<String>, Vec<Value>) {
    let paths = fs::read_dir(dir).unwrap();

    let mut toml = Vec::new();
    let mut json = Vec::new();

    for path in paths {
        let entry = path.unwrap();
        let extension = entry.path().extension().unwrap();
        match extension.as_ref() {
            "json" => json.push(serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap()),
            "toml" => toml.push(fs::read_to_string(path).unwrap()),
            _ => (),
        }
        println!("Name: {}", path.unwrap().path().display())
    }

    (toml, json)
}
