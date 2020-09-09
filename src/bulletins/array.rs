use crate::bulletins::num::handle_num;
use crate::bulletins::string::TomlString;
use crate::bulletins::TomlValue;
use crate::bulletins::{get_type, Types};
use crate::lexer::Tokens;
use crate::{assert_toml, comp_err};
use std::iter::Peekable;

pub struct Arr<'a, S: Iterator<Item = Tokens>> {
    peekable: &'a mut Peekable<S>,
    arr_type: Option<Types>,
}

#[derive(Debug)]
struct TomlArray<T> {
    array: Vec<T>,
}

#[macro_use]
macro_rules! close_or_comma {
    ( $arr : expr ) => {
        if let Some(Tokens::Sbc) = $arr.peekable.peek() {
            break;
        } else {
            assert_toml!($arr.peekable.next(), Tokens::Comma);
        }
    };
}

impl<'a, S: Iterator<Item = Tokens>> Arr<'a, S> {
    pub fn handle(peekable: &mut Peekable<S>) -> TomlValue {
        // determine the type of the array from the first element,
        // in toml, array elements can have only a single type
        let mut arr = Arr {
            peekable: peekable,
            arr_type: None,
        };
        let mut vec = Vec::new();
        while arr.peekable.peek().is_some() {
            if let Some(x) = arr.peekable.next() {
                match x {
                    Tokens::DoubleQuote | Tokens::SingleQuote => {
                        if !arr.has_type() {
                            arr.set_type(Types::String);
                            let mut string = TomlValue::String("".to_string());
                            if x == Tokens::DoubleQuote {
                                string = TomlString::handle(arr.peekable, 1);
                            } else {
                                string = TomlString::handle(arr.peekable, 3)
                            }
                            vec.push(string);
                            close_or_comma!(arr);
                        } else {
                            if arr.arr_type == Some(Types::String) {
                                let mut string = TomlValue::String("".to_string());
                                if x == Tokens::DoubleQuote {
                                    string = TomlString::handle(arr.peekable, 1);
                                } else {
                                    string = TomlString::handle(arr.peekable, 3)
                                }
                                vec.push(string);
                                close_or_comma!(arr);
                            } else {
                                if let Some(x) = arr.peekable.next() {
                                    panic!(
                                        "{:?} type found inside a {:?} type array",
                                        get_type(&x.to_string()),
                                        arr.arr_type
                                    );
                                }
                            }
                        }
                    }
                    Tokens::Sbo => {
                        let something = Arr::handle(arr.peekable);
                        close_or_comma!(arr);
                    }
                    // arrays can be spanned to multiple lines
                    Tokens::LineBreak => (),
                    _ => {
                        let type_of_literal = get_type(&x.to_string());
                        if !arr.has_type() {
                            arr.set_type(type_of_literal);
                            match type_of_literal {
                                Types::String => {
                                    comp_err!("Unexpected token, expected quotes or number")
                                }
                                Types::Int | Types::Floating => {
                                    let final_num = handle_num(x.to_string());
                                    vec.push(final_num);
                                    close_or_comma!(arr);
                                }
                                Types::Boolean => {
                                    vec.push(TomlValue::String(x.to_string()));
                                    close_or_comma!(arr);
                                }
                                _ => (),
                            }
                        } else {
                            let arr_type = arr.arr_type.unwrap();
                            if type_of_literal == arr_type {
                                vec.push(TomlValue::String(x.to_string()));
                                close_or_comma!(arr);
                            } else {
                                comp_err!(format!(
                                    "Found a {:?} type found in a {:?} type array",
                                    type_of_literal, arr_type
                                ));
                            }
                        }
                    }
                }
            }
        }
        assert_toml!(arr.peekable.next(), Tokens::Sbc);
        TomlValue::Array(vec)
    }

    pub fn has_type(&self) -> bool {
        self.arr_type.is_some()
    }
    pub fn set_type(&mut self, ele_type: Types) {
        self.arr_type = Some(ele_type);
    }
}
