use serde_json::{Result, Value};

use std::fs;
use std::os::unix::ffi::OsStrExt;

pub fn get_all_files(dir: &'static str) -> (Vec<String>, Vec<Value>) {
    let paths = fs::read_dir(dir).unwrap();

    let mut toml = Vec::new();
    let mut json = Vec::new();

    for path in paths {
        let entry = path.unwrap().path();
        let extension = entry.extension().unwrap();
        match extension.as_bytes() {
            b"json" => {
                json.push(serde_json::from_str(&fs::read_to_string(entry).unwrap()).unwrap())
            }
            b"toml" => toml.push(fs::read_to_string(entry).unwrap()),
            _ => (),
        }
    }

    (toml, json)
}
