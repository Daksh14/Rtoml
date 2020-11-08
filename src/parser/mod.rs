use crate::ast::{Ast, HeadOption, ParsingError};
use crate::lexer::Lexer;
use crate::TomlValue;
use std::collections::HashMap;
use std::convert::From;
use std::fs::{metadata, File};
use std::io::Read;

pub struct RToml {
    content: String,
    is_file: bool,
}

impl<'a> From<&'a str> for RToml {
    fn from(string: &'a str) -> Self {
        RToml {
            content: string.to_owned(),
            is_file: false,
        }
    }
}

impl RToml {
    pub fn file(filename: &str) -> Self {
        RToml {
            content: filename.to_owned(),
            is_file: true,
        }
    }
    pub fn parse(self) -> Result<Reader, ParsingError> {
        Reader::init(self)
    }
}

pub struct Reader {
    syntax_tree: Ast,
}

impl Reader {
    pub fn init(instance: RToml) -> Result<Self, ParsingError> {
        let mut data = Vec::new();
        if instance.is_file {
            data.extend(Self::read_file(instance.content))
        } else {
            data = instance.content.as_bytes().to_vec();
        }
        let lexical = Lexer::lex(data);
        Ok(Reader {
            syntax_tree: Ast::make(lexical)?,
        })
    }
    fn read_file(file_name: String) -> Vec<u8> {
        let meta_data = metadata(file_name.clone()).ok().unwrap();
        let mut data = Vec::with_capacity(meta_data.len() as usize);

        let mut file = File::open(file_name).expect("Unable to open file");
        file.read_to_end(&mut data).expect("Unable to read data");
        data
    }
    pub fn get_table(&self, table_name: &str) -> HashMap<String, TomlValue> {
        let current_child = &self.syntax_tree.child;
        let mut val = HashMap::new();
        for nodes in current_child.iter() {
            if let HeadOption::Some(x) = &nodes.item {
                if x.get_table() == Some(&table_name.to_string()) {
                    for key_values in x.key_value.clone().into_iter() {
                        val.insert(key_values.0, key_values.1);
                    }
                    break;
                }
            }
        }
        val
    }
    pub fn get_table_array(&self, table_name: &str) -> HashMap<String, Vec<TomlValue>> {
        let current_child = &self.syntax_tree.child;
        let mut val = HashMap::new();
        for nodes in current_child.into_iter() {
            if let HeadOption::TableArr(x) = &nodes.item {
                if x == table_name {
                    if let HeadOption::Some(content) = &nodes.child[0].item {
                        for values in content.key_value.iter() {
                            if let TomlValue::Array(y) = values.1 {
                                val.insert(values.0.to_string(), y.clone());
                            }
                        }
                    }
                }
            }
        }
        val
    }
}
