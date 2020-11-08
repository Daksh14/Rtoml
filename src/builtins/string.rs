use crate::builtins::{ParsingError, TomlValue};
use crate::lexer::Tokens;
use crate::{assert_toml, comp_err};
use std::iter::Peekable;

pub struct TomlString<'a, S: Iterator<Item = Tokens>> {
    peekable: &'a mut Peekable<S>,
}

impl<'a, S: Iterator<Item = Tokens>> TomlString<'a, S> {
    pub fn handle(peekable: &mut Peekable<S>, quote_count: u8) -> Result<TomlValue, ParsingError> {
        let mut string = TomlString { peekable };
        string.check_quotes();

        match quote_count {
            1 => {
                let value = string.parse()?;
                assert_toml!(peekable.next(), Tokens::DoubleQuote);
                Ok(TomlValue::String(value))
            }
            3 => {
                let value = string.parse()?;
                for _ in 0..2 {
                    assert_toml!(peekable.next(), Tokens::DoubleQuote);
                }
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
                if x == Tokens::LineBreak {
                    return Err(ParsingError::Expected(
                        Tokens::DoubleQuote.to_string(),
                        Tokens::LineBreak.to_string(),
                    ));
                }
                value.push_str(&x.to_string());
            } else {
                return Err(ParsingError::Expected(
                    Tokens::DoubleQuote.to_string(),
                    "None".to_string(),
                ));
            }
        }
        Ok(value)
    }
    pub fn parse_triple_quote(&mut self) -> Result<String, ParsingError> {
        let mut value = String::new();
        while Some(&Tokens::DoubleQuote) != self.peekable.peek() {
            if let Some(x) = self.peekable.next() {
                value.push_str(&x.to_string());
            } else {
                return Err(ParsingError::Expected(
                    Tokens::DoubleQuote.to_string(),
                    "None".to_string(),
                ));
            }
        }
        Ok(value)
    }
    pub fn check_quotes(&mut self) {
        if let Some(Tokens::DoubleQuote) | Some(Tokens::TripleDoubleQuotes) = self.peekable.peek() {
            comp_err!("Invalid number of quotes");
        }
    }
}
