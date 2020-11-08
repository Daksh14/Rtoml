use crate::ast::context::{Context, TableArrayContext};
use crate::builtins::Types;
use crate::lexer::Tokens;
use crate::TomlValue;
use std::error::Error;
use std::fmt;

pub mod context;

#[derive(Debug, PartialEq, Eq)]
pub enum HeadOption<T> {
    Some(T),
    None,
    // Table array with the array name
    TableArr(String),
    Head,
}

impl<T> HeadOption<T> {
    pub fn is_some(&self) -> bool {
        !matches!(self, HeadOption::None)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ParsingError {
    MissingIdentifier(String),
    // Expected .0 found .1
    Expected(String, String),
    StringErr(u8, String),
    // wrong type.0, correct type.1
    TypeError(String, String),
}

#[derive(Debug)]
pub struct Ast {
    pub child: Vec<Ast>,
    pub item: HeadOption<Context>,
}

#[macro_export]
macro_rules! comp_err {
    ( $expected : expr , $found : expr) => {
        panic!(
            "Compiling error, expected {:?}, found {:?}",
            $expected, $found
        );
    };
    ( $message : expr) => {
        panic!($message);
    };
}

#[macro_export]
macro_rules! assert_toml {
    ( $left : expr , $right : expr) => {
        if let Some(x) = $left {
            if x == $right {
                ()
            } else {
                return Err(ParsingError::Expected($right.to_string(), x.to_string()));
            }
        } else {
            return Err(ParsingError::Expected(
                $right.to_string(),
                "None".to_string(),
            ));
        }
    };
}

#[macro_export]
macro_rules! close_or_comma {
    ( $peekable : expr, $close_at : pat) => {{
        use crate::assert_toml;
        if let Some($close_at) = $peekable.peek() {
            break;
        } else {
            assert_toml!($peekable.next(), Tokens::Comma);
        }
    }};
}

impl Ast {
    pub fn make(lexemes: Vec<Tokens>) -> Result<Self, ParsingError> {
        let mut peekable = lexemes.into_iter().peekable();
        let mut head = Ast::new_head();
        let mut context = Context::new(None);
        let mut table_array = TableArrayContext::new();
        while peekable.peek().is_some() {
            match peekable.peek() {
                // table declaration
                Some(Tokens::Sbo) => {
                    peekable.next();
                    match peekable.peek() {
                        Some(Tokens::Literal(_)) => {
                            table_array.make_context_invalid();
                            if context.has_context() {
                                head.child.push(Ast::new_context(context.clone()));
                            }
                            let literal = peekable.next().unwrap().to_string();
                            context.change_context(literal);
                            assert_toml!(peekable.next(), Tokens::Sbc);
                        }
                        Some(Tokens::Sbo) => {
                            peekable.next();
                            if let Some(Tokens::Literal(_)) = peekable.peek() {
                                if !table_array.context_is_valid() {
                                    head.child.push(Ast::new_context(context.clone()));
                                }
                                table_array.make_context_valid();
                                let literal = peekable.next().unwrap().to_string();
                                context.change_context(literal);
                            } else {
                                return Err(ParsingError::Expected(
                                    "Literal".to_string(),
                                    peekable.next().unwrap().to_string(),
                                ));
                            }
                        }
                        _ => (),
                    }
                }
                Some(Tokens::Hash) => {
                    peekable.next();
                    while peekable.peek().is_some() {
                        if let Some(Tokens::LineBreak) = peekable.next() {
                            break;
                        }
                    }
                }
                Some(Tokens::Literal(_)) => {
                    let identifier = peekable.next().unwrap().to_string();
                    if let Some(x) = identifier.chars().next() {
                        if x.is_numeric() {
                            return Err(ParsingError::Expected(
                                "String".to_string(),
                                "Number".to_string(),
                            ));
                        }
                    } else {
                        return Err(ParsingError::Expected(
                            "String".to_string(),
                            "None".to_string(),
                        ));
                    }
                    assert_toml!(peekable.next(), Tokens::Eq);
                    let value = Types::handle_value(&mut peekable)?;
                    if !table_array.context_is_valid() {
                        context.set_value(&identifier, value);
                    } else if let Some(ref x) = context.table_name {
                        table_array.set_value(&identifier, value, x.to_string());
                    }
                }
                _ => {
                    peekable.next();
                }
            }
        }
        if context.has_context() {
            head.child.push(Ast::new_context(context));
        }

        head.child.extend(Ast::new_table_array(table_array));
        Ok(head)
    }
    pub fn new_head() -> Self {
        Ast {
            child: Vec::new(),
            item: HeadOption::Head,
        }
    }

    pub fn new_context(context: Context) -> Self {
        Ast {
            child: Vec::new(),
            item: HeadOption::Some(context),
        }
    }
    pub fn new_table_array(context: TableArrayContext) -> Vec<Self> {
        let mut tables = Vec::new();
        for each_tables in context.tables.into_iter() {
            let table_name = each_tables.0;
            let mut child = Vec::new();
            let mut context = Context::new(Some(table_name.clone()));
            for key_values in each_tables.1.into_iter() {
                context.set_value(&key_values.0, TomlValue::Array(key_values.1));
            }
            child.push(Ast {
                child: Vec::new(),
                item: HeadOption::Some(context),
            });
            let instances = Ast {
                child,
                item: HeadOption::TableArr(table_name),
            };
            tables.push(instances);
        }
        tables
    }
    pub fn has_token(&self) -> bool {
        self.item.is_some()
    }
}

impl Error for ParsingError {}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let clause = match self {
            ParsingError::MissingIdentifier(x) => format!("Missing identifier {}", x),
            ParsingError::TypeError(x, y) => format!("Found {} type in array, expected {}", x, y),
            ParsingError::Expected(x, y) => format!("Expected {} found {}", x, y),
            ParsingError::StringErr(x, y) => match x {
                0 => format!("Invalid escaped char {}", y),
                1 => format!("Unicode character is a non scalar value {}", y),
                _ => Default::default(),
            },
        };
        write!(f, "{}", clause)
    }
}
