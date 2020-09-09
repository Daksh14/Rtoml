use crate::ast::Node;
use crate::ast::TomlValue;
use crate::bulletins::array::Arr;
use crate::bulletins::inline_table::parse_inline_table;
use crate::bulletins::num::check_num;
use crate::bulletins::num::handle_num;
use crate::bulletins::string::TomlString;
use crate::comp_err;
use crate::lexer::Tokens;
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

pub fn get_type(value: &String) -> Types {
    let num_type = check_num(value);
    if let Types::Void = num_type {
        match value.to_lowercase().as_str() {
            "true" | "false" => Types::Boolean,
            _ => Types::String,
        }
    } else {
        num_type
    }
}

pub fn handle_value<S: Iterator<Item = Tokens>>(peekable: &mut Peekable<S>) -> Node {
    match peekable.next() {
        // A string
        Some(Tokens::DoubleQuote) => Node::Value(TomlString::handle(peekable, 1)),
        Some(Tokens::TripleDoubleQuotes) => Node::Value(TomlString::handle(peekable, 3)),
        // an array
        Some(Tokens::Sbo) => Node::Value(Arr::handle(peekable)),
        // inline table
        //  Some(Tokens::Cbo) => parse_inline_table(peekable),
        // Litearlly anything else
        Some(Tokens::Literal(x)) => {
            let value = match get_type(&x) {
                Types::Int | Types::Floating => handle_num(x.to_string()),
                Types::Boolean => match x.to_lowercase().as_str() {
                    "true" => TomlValue::Boolean(true),
                    "false" => TomlValue::Boolean(false),
                    _ => comp_err!("Invalid boolean type"),
                },
                Types::String | Types::Void => comp_err!("Invalid character found"),
                _ => comp_err!("Datetime not supported"),
            };
            Node::Value(value)
        }
        None => comp_err!("Expected value, found None"),
        _ => comp_err!("Expected value, found unrecognised token"),
    }
}
