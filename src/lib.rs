use std::collections::HashMap;

mod ast;
mod builtins;
mod lexer;
pub mod parser;

#[derive(Debug, Clone, PartialEq)]
pub enum TomlValue {
    Int(i64),
    Floating(f64),
    String(String),
    Array(Vec<TomlValue>),
    Boolean(bool),
    InlineTable(HashMap<String, TomlValue>),
}

impl TomlValue {
    pub fn get_int(&self) -> Option<i64> {
        if let Self::Int(x) = self {
            Some(*x)
        } else {
            None
        }
    }
    pub fn get_floating(&self) -> Option<f64> {
        if let Self::Floating(x) = self {
            Some(*x)
        } else {
            None
        }
    }
    pub fn get_string(&self) -> Option<String> {
        if let Self::String(x) = self {
            Some(x.to_string())
        } else {
            None
        }
    }
    pub fn get_array(&self) -> Option<Vec<TomlValue>> {
        if let Self::Array(x) = self {
            Some(x.to_vec())
        } else {
            None
        }
    }
    pub fn get_boolean(&self) -> Option<bool> {
        if let Self::Boolean(x) = self {
            Some(*x)
        } else {
            None
        }
    }
    pub fn get_inline_table(&self) -> Option<&HashMap<String, TomlValue>> {
        if let Self::InlineTable(x) = self {
            Some(x)
        } else {
            None
        }
    }
}

pub mod prelude {
    pub use crate::parser::RToml;
    pub use crate::TomlValue::*;
}
