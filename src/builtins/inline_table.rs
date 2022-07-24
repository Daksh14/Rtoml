use crate::builtins::parse_value;
use crate::lexer::Token;
use crate::parser::r_iter::RIter;
use crate::parser::r_slice::RSlice;
use crate::{TomlError, TomlValue};

use crate::error::ErrLocation;
use rustc_hash::FxHashMap;

pub fn parse_inline_table(slice: RSlice) -> Result<TomlValue, TomlError> {
    let mut iter = RIter::from(slice);
    let mut map = FxHashMap::default();

    while let Some((token, _)) = iter.next() {
        match token {
            Token::Literal(str) => {
                if Token::Literal(str).is_valid_table_name_or_key() {
                    if let Some((eq, _)) = iter.next() {
                        if *eq == Token::Eq {
                            map.insert(str.trim(), parse_value(iter.as_slice())?);
                        }
                    }
                }
            }
            Token::Cbc => {
                break;
            }
            _ => {
                return Err(TomlError::UnexpectedCharacter(
                    ErrLocation::new(iter),
                    &[Token::Cbc, Token::Literal("KEY")],
                ))
            }
        }
    }

    Ok(TomlValue::Table(map))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;
    use crate::TomlValue;

    use rustc_hash::FxHashMap;

    #[test]
    fn basic_inline_table() {
        let mut map = FxHashMap::default();
        map.insert("value", TomlValue::Int(1));
        let table = &lex(b"value = 1 }").unwrap();
    }
}
