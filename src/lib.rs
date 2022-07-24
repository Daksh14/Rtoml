extern crate core;

use crate::error::TomlError;

use std::convert::TryFrom;

use rustc_hash::FxHashMap;

mod builtins;
mod lexer;
mod parser;

pub mod error;
pub mod prelude {
    pub use crate::TomlValue::*;
}

#[derive(Debug, Clone, PartialEq)]
pub enum TomlValue<'a> {
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<TomlValue<'a>>),
    Boolean(bool),
    DateTime(DateTime),
    Table(FxHashMap<&'a str, TomlValue<'a>>),
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

    pub fn as_table(&self) -> Option<&FxHashMap<&str, TomlValue>> {
        extract!(self, Table)
    }

    pub fn from_str<'b: 'a>(str: &'b str) -> Result<TomlValue, TomlError> {
        let lexed = lexer::lex(str.as_bytes())?;
        parser::parse(lexed)
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
