use crate::assert_toml;
use crate::builtins::{ParsingError, TomlValue};
use crate::lexer::Tokens;
use std::iter::Peekable;

pub struct TomlString<'a, S: Iterator<Item = Tokens>> {
    peekable: &'a mut Peekable<S>,
}

impl<'a, S: Iterator<Item = Tokens>> TomlString<'a, S> {
    pub fn handle(peekable: &mut Peekable<S>, quote_count: u8) -> Result<TomlValue, ParsingError> {
        let mut string = TomlString { peekable };
        string.check_quotes()?;

        match quote_count {
            1 => {
                let value = string.parse()?;
                assert_toml!(peekable.next(), Tokens::DoubleQuote);
                Ok(TomlValue::String(value))
            }
            3 => {
                let value = string.parse_triple_quote()?;
                assert_toml!(peekable.next(), Tokens::TripleDoubleQuotes);
                Ok(TomlValue::String(value))
            }
            _ => Err(ParsingError::Expected(
                "3 or 1 quotes".to_string(),
                quote_count.to_string(),
            )),
        }
    }
    pub fn parse(&mut self) -> Result<String, ParsingError> {
        let mut value = String::new();
        while Some(&Tokens::DoubleQuote) != self.peekable.peek() {
            if let Some(x) = self.peekable.next() {
                if x.to_string() == "\\" {
                    self.escape(&mut value)?;
                } else {
                    value.push_str(&x.to_string());
                }

                if x == Tokens::LineBreak {
                    return Err(ParsingError::Expected(
                        Tokens::DoubleQuote.to_string(),
                        Tokens::LineBreak.to_string(),
                    ));
                }
            } else {
                return Err(ParsingError::Expected(
                    Tokens::DoubleQuote.to_string(),
                    "None".to_string(),
                ));
            }
        }
        Ok(value)
    }
    pub fn escape(&mut self, value: &mut String) -> Result<(), ParsingError> {
        if let Some(x) = self.peekable.next() {
            let val = x.to_string();
            match val.as_str() {
                // backspace
                "b" => value.push(0x08 as char),
                // tab
                "t" => value.push(0x09 as char),
                // linefeed
                "n" => value.push(0x0A as char),
                // form feed
                "f" => value.push(0x0C as char),
                // quote
                r#"""# => value.push(0x22 as char),
                // carriage return
                "r" => value.push(0x0D as char),
                // backslash
                "\\" => value.push(0x5C as char),
                // unicode
                "u" => {
                    let mut integer = String::with_capacity(4);
                    for _ in 0..3 {
                        if let Some(x) = self.peekable.peek() {
                            if let Ok(y) = x.to_string().parse::<u8>() {
                                integer.push_str(&y.to_string());
                                self.peekable.next();
                            }
                        }
                    }
                    if let Some(x) = self.peekable.peek() {
                        if let Ok(y) = x.to_string().parse::<u8>() {
                            integer.push_str(&y.to_string());
                            self.peekable.next();
                        }
                    }
                    for _ in 0..2 {
                        if let Some(x) = self.peekable.peek() {
                            if let Ok(y) = x.to_string().parse::<u8>() {
                                integer.push_str(&y.to_string());
                                self.peekable.next();
                            }
                        }
                    }
                    println!("{:?}", integer);
                    if let Ok(x) = integer.parse::<u32>() {
                        if let Some(y) = std::char::from_u32(x) {
                            println!("{:?}", y);
                            value.push(y);
                        } else {
                            return Err(ParsingError::StringErr(1, integer));
                        }
                    }
                }
                _ => return Err(ParsingError::StringErr(0, val.to_string())),
            }
        }
        Ok(())
    }
    pub fn parse_triple_quote(&mut self) -> Result<String, ParsingError> {
        let mut value = String::new();
        while Some(&Tokens::TripleDoubleQuotes) != self.peekable.peek() {
            if let Some(x) = self.peekable.next() {
                if x.to_string() == "\\" {
                    self.escape(&mut value)?;
                } else {
                    value.push_str(&x.to_string());
                }
            } else {
                return Err(ParsingError::Expected(
                    Tokens::DoubleQuote.to_string(),
                    "None".to_string(),
                ));
            }
        }
        Ok(value)
    }
    pub fn check_quotes(&mut self) -> Result<(), ParsingError> {
        if let Some(Tokens::DoubleQuote) | Some(Tokens::TripleDoubleQuotes) = self.peekable.peek() {
            Err(ParsingError::Expected(
                "3 or 1 quotes".to_string(),
                "Invalid number of quotes".to_string(),
            ))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    pub fn basic_string_single_quote() {
        let string = br#""hello, world""#;
        let lexial = Lexer::lex(string.to_vec());
        let mut peekable = lexial.into_iter().peekable();
        let string = TomlString::handle(&mut peekable, 1).unwrap();
        assert_eq!(string, TomlValue::String(String::from("hello, world")));
    }
}
