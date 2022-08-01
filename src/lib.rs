//! `RToml`
extern crate core;

use crate::error::TomlError;
use crate::lexer::lex;
use crate::parser::r_iter::RIter;
use crate::parser::ParsedValue;

use std::convert::TryFrom;
use std::fmt::{write, Display, Formatter};

use rustc_hash::FxHashMap;

mod builtins;
mod lexer;
mod parser;

pub mod error;
pub mod prelude {
    pub use crate::error::TomlError;
    pub use crate::{DateTime, Table, TomlKey, TomlValue};
}

pub type Table<'a> = FxHashMap<TomlKey<'a>, TomlValue<'a>>;

#[derive(Debug, Clone, PartialEq)]
pub enum TomlValue<'a> {
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<TomlValue<'a>>),
    Boolean(bool),
    DateTime(DateTime),
    Table(Table<'a>),
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum TomlKey<'a> {
    Literal(&'a str),
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DateTime {
    DateTime(speedate::DateTime),
    Date(speedate::Date),
    Time(speedate::Time),
}

macro_rules! extract {
    ( $self : expr, $clause : ident ) => {
        if let Self::$clause(x) = $self {
            Some(x)
        } else {
            None
        }
    };
}

impl<'a> TomlValue<'a> {
    pub fn as_int(&self) -> Option<&i64> {
        extract!(self, Int)
    }

    pub fn as_floating(&self) -> Option<&f64> {
        extract!(self, Float)
    }

    pub fn as_string(&self) -> Option<&String> {
        extract!(self, String)
    }

    pub fn as_array(&self) -> Option<&Vec<TomlValue<'a>>> {
        extract!(self, Array)
    }

    pub fn as_boolean(&self) -> Option<&bool> {
        extract!(self, Boolean)
    }

    pub fn as_datetime(&self) -> Option<&DateTime> {
        extract!(self, DateTime)
    }

    pub fn as_table(&self) -> Option<&Table> {
        extract!(self, Table)
    }
}

impl<'a> TomlKey<'a> {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

impl DateTime {
    pub fn as_datetime(&self) -> Option<&speedate::DateTime> {
        extract!(self, DateTime)
    }

    pub fn as_date(&self) -> Option<&speedate::Date> {
        extract!(self, Date)
    }

    pub fn as_time(&self) -> Option<&speedate::Time> {
        extract!(self, Time)
    }
}

fn hash_map_pretty_print(hashmap: &Table, padding_level: usize) -> String {
    let padding = "  ".repeat(padding_level);
    let mut key_value = String::new();
    let mut braces = String::new();
    let mut table_name = &TomlKey::None;

    for (key, value) in hashmap.iter() {
        if let Some(table) = value.as_table() {
            table_name = key;
            key_value.push_str(hash_map_pretty_print(table, padding_level + 1).as_str());
        } else {
            key_value.push_str(format!("{}{}: {},\n", padding.repeat(2), key, value).as_str());
        }
    }

    if table_name.is_none() {
        braces.push_str(format!("{}{{\n", padding).as_str());
        braces.push_str(&key_value);
        braces.push_str(format!("{}}}\n", padding).as_str());
    } else {
        braces.push_str(format!("{}{}: {{\n", padding, table_name).as_str());
        braces.push_str(&key_value);
        braces.push_str(format!("{}}}\n", padding).as_str());
    }

    braces
}

impl Display for TomlValue<'_> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TomlValue::Int(x) => fmt.write_str(x.to_string().as_str()),
            TomlValue::Float(x) => fmt.write_str(x.to_string().as_str()),
            TomlValue::String(x) => write!(fmt, "{:?}", x),
            TomlValue::Boolean(x) => fmt.write_str(x.to_string().as_str()),
            TomlValue::DateTime(x) => fmt.write_str(x.to_string().as_str()),
            TomlValue::Array(x) => {
                fmt.write_str("[")?;
                for value in x {
                    fmt.write_str(value.to_string().as_str())?;
                    fmt.write_str(", ")?;
                }
                fmt.write_str("\x08\x08]")
            }
            TomlValue::Table(x) => {
                writeln!(fmt, "{}", hash_map_pretty_print(x, 0))
            }
        }
    }
}

impl<'a> TryFrom<&'a str> for TomlValue<'a> {
    type Error = TomlError<'a>;

    fn try_from(str: &'a str) -> Result<Self, Self::Error> {
        let lexed = lex(str.as_bytes())?.leak();
        ParsedValue::new(TomlValue::Int(0), RIter::new(lexed)).parse()
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let x = match self {
            DateTime::DateTime(x) => format!("{}", x),
            DateTime::Date(x) => format!("{}", x),
            DateTime::Time(x) => format!("{}", x),
        };

        write!(f, "{}", x)
    }
}

impl Display for TomlKey<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Self::None => format!(""),
            Self::Literal(x) => format!("{}", x),
        };

        write!(f, "{}", val)
    }
}

impl<'a> From<&'a str> for TomlKey<'a> {
    fn from(str: &'a str) -> Self {
        Self::Literal(str)
    }
}
