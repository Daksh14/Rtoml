use crate::ast::Ast;
use crate::ast::HeadOption;
use crate::ast::Node;
use crate::lexer::Lexer;
use std::collections::HashMap;
use std::fs::metadata;
use std::fs::File;
use std::io::Read;

pub struct RToml {
    filename: String,
}

impl RToml {
    pub fn file(filename: &str) -> Self {
        RToml {
            filename: filename.to_owned(),
        }
    }
    pub fn parse(self) -> std::io::Result<Reader> {
        Reader::init(self)
    }
}

pub struct Reader {
    syntax_tree: Ast,
}

impl Reader {
    pub fn init(instance: RToml) -> std::io::Result<Self> {
        let file_name = instance.filename;
        let meta_data = metadata(file_name.clone())?;
        let mut data = Vec::with_capacity(meta_data.len() as usize);
        let mut file = File::open(file_name).expect("Unable to open file");
        file.read_to_end(&mut data).expect("Unable to read data");
        let lexical = Lexer::lex(data);
        Ok(Reader {
            syntax_tree: Ast::make(lexical),
        })
    }
    pub fn get_table(&self, table_name: &str) -> HashMap<String, Node> {
        let mut current_child = &self.syntax_tree.child;
        let mut val = HashMap::new();
        for nodes in current_child.into_iter() {
            if let HeadOption::Some(x) = &nodes.item {
                if x.get_table() == Some(&table_name.to_string()) {
                    val = x.key_value.clone();
                    break;
                }
            }
        }
        val
    }
}
