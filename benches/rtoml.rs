use std::convert::TryFrom;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::string::String;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rtoml::prelude::*;
use toml::Value;

fn rtoml_parse() {
    let mut data = String::new();
    let mut file = File::open(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/test.toml")).unwrap();

    file.read_to_string(&mut data).unwrap();

    let _ = TomlValue::try_from(data.as_str()).unwrap();
}

fn serde_toml_parse() {
    let mut data = String::new();
    let mut file = File::open(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/test.toml")).unwrap();

    file.read_to_string(&mut data).unwrap();

    let _ = data.parse::<Value>().unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Rtoml parsing", |b| b.iter(rtoml_parse));
    c.bench_function("serde toml parsing", |b| b.iter(serde_toml_parse));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
