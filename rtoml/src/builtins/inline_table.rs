use crate::builtins::parse_value;
use crate::lexer::Token;
use crate::parser::r_iter::RIter;
use crate::parser::r_slice::RSlice;
use crate::{TomlError, TomlValue};

use crate::error::ErrLocation;
use crate::parser::ParsedValue;
use rustc_hash::FxHashMap;

pub fn parse_inline_table(slice: RSlice) -> Result<ParsedValue, TomlError> {
    let mut iter = RIter::from(slice);
    let mut map = FxHashMap::default();

    while let Some((token, _)) = iter.next() {
        match token {
            Token::Literal(_) => {
                if token.is_valid_table_name_or_key() && iter.next_if_eq(Token::Eq) {
                    let parsed = parse_value(iter.as_slice())?;
                    map.insert(token.as_key(), parsed.value);
                    iter = parsed.section;
                }
            }
            Token::Cbc => {
                break;
            }
            Token::Comma => {
                continue;
            }
            _ => {
                return Err(TomlError::UnexpectedCharacter(
                    ErrLocation::new(iter),
                    &[Token::Cbc, Token::Literal("")],
                ));
            }
        }
    }

    Ok(ParsedValue::new(TomlValue::Table(map), iter))
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
