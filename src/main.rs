use crate::lexer::lex;
use rtoml::init::RToml;
use rtoml::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::string::String;

mod lexer;

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("templates/test.toml").expect("Error opening file");
    let mut data = String::new();

    file.read_to_string(&mut data);

    println!("TOML \n {} \n", data);
    println!("LEXED \n {:?} \n ", lex(data.as_bytes()));

    println!("AST \n {:?} \n ", RToml::from(String::from(data)).parse());

    Ok(())
}
