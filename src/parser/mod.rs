use crate::builtins::parse_value;
use crate::error::ErrLocation;
use crate::lexer::{Token, TokenSized};
use crate::parser::r_iter::RIter;
use crate::{TomlError, TomlValue};

pub mod r_iter;
pub mod r_slice;

pub fn parse(lexemes: Vec<TokenSized>) -> Result<TomlValue, TomlError> {
    let mut iter = RIter::new(&lexemes);
    let mut value = TomlValue::Int(0);

    while let Some(token) = iter.next() {
        match token {
            (Token::Sbo, _) => {
                if let Some((next, _)) = iter.next() {
                    if *next == Token::Sbo {
                        // table array segment
                    } else if next.is_valid_table_name_or_key() {
                        if !iter.next_if_eq(&Token::Sbc) {
                            return Err(TomlError::UnexpectedCharacter(
                                ErrLocation::new(RIter::from(iter.as_slice())),
                                &[Token::Sbc],
                            ));
                        }

                        if let Some((not_space, _)) = iter.next() {
                            if not_space.is_space() {
                                iter.next();
                            }

                            if !iter.next_if_eq(&Token::LineBreak) {
                                return Err(TomlError::UnexpectedCharacter(
                                    ErrLocation::new(RIter::from(iter.as_slice())),
                                    &[Token::LineBreak],
                                ));
                            }
                        }

                        if let Some((literal, _)) = iter.next() {
                            if literal.is_valid_table_name_or_key() {
                                if !iter.next_if_eq(&Token::Eq) {
                                    return Err(TomlError::UnexpectedCharacter(
                                        ErrLocation::new(RIter::from(iter.as_slice())),
                                        &[Token::Eq],
                                    ));
                                }
                                value = parse_value(iter.as_slice())?;
                            }
                        }
                    }
                }
            }
            // Top level key and value declaration
            (Token::Literal(x), _) => {
                if Token::Literal(x).is_valid_table_name_or_key() {
                    if !iter.next_if_eq(&Token::Eq) {
                        return Err(TomlError::UnexpectedCharacter(
                            ErrLocation::new(RIter::from(iter.as_slice())),
                            &[Token::Eq],
                        ));
                    }
                    value = parse_value(iter.as_slice())?;
                }
            }
            _ => (),
        }
    }

    Ok(value)
}
