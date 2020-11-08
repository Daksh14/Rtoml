use crate::ast::ParsingError;
use crate::builtins::array::TomlArray;
use crate::builtins::inline_table::TomlInlineTable;
use crate::builtins::num::TomlNum;
use crate::builtins::string::TomlString;
use crate::comp_err;
use crate::lexer::Tokens;
use crate::TomlValue;
use std::fmt;
use std::iter::Peekable;

pub mod array;
pub mod inline_table;
pub mod num;
pub mod string;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Types {
    Int,
    Floating,
    String,
    Boolean,
    DateTime,
    Void,
}

impl fmt::Display for Types {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let val = match self {
            Types::Int => "Integer",
            Types::Floating => "Floating",
            Types::String => "String",
            Types::Boolean => "Boolean",
            Types::DateTime => "Datetime",
            Types::Void => "Void",
        };
        write!(f, "{}", val)
    }
}

impl Types {
    pub fn get_type(value: &str) -> Result<Self, ParsingError> {
        let num_type = TomlNum::check_num(value)?;
        if let Types::Void = num_type {
            match value.to_lowercase().as_str() {
                "true" | "false" => Ok(Types::Boolean),
                _ => Ok(Types::String),
            }
        } else {
            Ok(num_type)
        }
    }
    pub fn handle_value<S: Iterator<Item = Tokens>>(
        peekable: &mut Peekable<S>,
    ) -> Result<TomlValue, ParsingError> {
        match peekable.next() {
            // A string
            Some(Tokens::DoubleQuote) => TomlString::handle(peekable, 1),
            Some(Tokens::TripleDoubleQuotes) => TomlString::handle(peekable, 3),
            // an array
            Some(Tokens::Sbo) => TomlArray::handle(peekable),
            // inline table
            Some(Tokens::Cbo) => TomlInlineTable::handle(peekable),
            // Litearlly anything else
            Some(Tokens::Literal(x)) => {
                let value = match Self::get_type(&x)? {
                    Types::Int | Types::Floating => TomlNum::handle(x.to_string()),
                    Types::Boolean => TomlValue::Boolean(x.parse::<bool>().unwrap()),
                    Types::String | Types::Void => comp_err!("Invalid character found"),
                    _ => comp_err!("Datetime not supported"),
                };
                Ok(value)
            }
            None => Err(ParsingError::Expected(
                "Value".to_string(),
                "None".to_string(),
            )),
            _ => Err(ParsingError::Expected(
                "Value".to_string(),
                "Invalid Token".to_string(),
            )),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn bool_type() {
        assert_eq!(Types::Boolean, Types::get_type("true").unwrap());
        assert_eq!(Types::Boolean, Types::get_type("false").unwrap());
    }
}
