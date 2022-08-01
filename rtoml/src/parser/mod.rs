use crate::builtins::parse_value;
use crate::error::ErrLocation;
use crate::lexer::Token;
use crate::parser::r_iter::RIter;
use crate::{Table, TomlError, TomlKey, TomlValue};

use rustc_hash::FxHashMap;

pub mod r_iter;
pub mod r_slice;

pub struct ParsedValue<'a> {
    pub value: TomlValue<'a>,
    pub section: RIter<'a>,
}

impl<'a> ParsedValue<'a> {
    pub fn new(value: TomlValue<'a>, section: RIter<'a>) -> Self {
        Self { value, section }
    }

    pub fn parse(self) -> Result<TomlValue<'a>, TomlError<'a>> {
        let mut iter = self.section;
        let mut value = FxHashMap::default();

        while let Some(token) = iter.next() {
            match token {
                (Token::Sbo, _) => {
                    if let Some((next, _)) = iter.next() {
                        if *next == Token::Sbo {
                            // table array segment
                        } else if next.is_valid_table_name_or_key() {
                            if !iter.next_if_eq(Token::Sbc) {
                                return Err(TomlError::UnexpectedCharacter(
                                    ErrLocation::new(RIter::from(iter.as_slice())),
                                    &[Token::Sbc],
                                ));
                            }

                            while let Some((linebreak, _)) = iter.peek() {
                                if Token::LineBreak == *linebreak {
                                    iter.next();
                                    break;
                                } else if linebreak.is_space() {
                                    continue;
                                } else {
                                    return Err(TomlError::UnexpectedCharacter(
                                        ErrLocation::new(RIter::from(iter.as_slice())),
                                        &[Token::Eq],
                                    ));
                                }
                            }

                            let mut table_content = FxHashMap::default();

                            while let Some((token, _)) = iter.next() {
                                Self::key_value(token, &mut iter, &mut table_content)?;

                                if *token == Token::LineBreak {
                                    if let Some((Token::Sbo, _)) = iter.peek() {
                                        break;
                                    }
                                }
                            }

                            value.insert(next.as_key(), TomlValue::Table(table_content));
                        }
                    }
                }
                // Top level key and value declaration
                (Token::Literal(_), _) => {
                    let mut table_content = FxHashMap::default();

                    Self::key_value(&token.0, &mut iter, &mut table_content)?;

                    while let Some((token, _)) = iter.next() {
                        if *token == Token::LineBreak {
                            if let Some((Token::Sbo, _)) = iter.peek() {
                                break;
                            }
                        }

                        Self::key_value(token, &mut iter, &mut table_content)?;
                    }

                    value.insert(TomlKey::None, TomlValue::Table(table_content));
                }
                _ => (),
            }
        }

        Ok(TomlValue::Table(value))
    }

    pub fn key_value(
        token: &'a Token<'a>,
        iter: &mut RIter<'a>,
        table_content: &mut Table<'a>,
    ) -> Result<(), TomlError<'a>> {
        if token.is_valid_table_name_or_key() && !token.is_space() {
            if !iter.next_if_eq(Token::Eq) {
                return Err(TomlError::UnexpectedCharacter(
                    ErrLocation::new(RIter::from(iter.as_slice())),
                    &[Token::Eq],
                ));
            }

            let parsed = parse_value(iter.as_slice())?;
            *iter = parsed.section;

            table_content.insert(token.as_key(), parsed.value);
        }

        Ok(())
    }
}
