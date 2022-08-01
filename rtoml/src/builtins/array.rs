use crate::builtins::parse_value;
use crate::error::ErrLocation;
use crate::lexer::Token;
use crate::parser::r_iter::RIter;
use crate::parser::r_slice::RSlice;
use crate::parser::ParsedValue;
use crate::{TomlError, TomlValue};

pub fn parse_array(slice: RSlice) -> Result<ParsedValue, TomlError> {
    let mut iter = RIter::from(slice);
    let mut vec = Vec::new();

    while let Some((next, _)) = iter.peek() {
        match next {
            Token::Sbc => {
                break;
            }
            Token::Comma => {
                return Err(TomlError::UnexpectedCharacter(
                    ErrLocation::new(RIter::from(iter.as_slice())),
                    &[Token::Sbc, Token::Literal("")],
                ));
            }
            Token::LineBreak => {
                iter.next();
            }
            n if n.is_space() => {
                iter.next();
            }
            _ => {
                let parsed = parse_value(iter.as_slice())?;
                vec.push(parsed.value);
                iter = parsed.section;

                if !iter.next_if_eq(Token::Comma) {
                    if !iter.next_if_eq(Token::Sbc) {
                        return Err(TomlError::UnexpectedCharacter(
                            ErrLocation::new(RIter::from(iter.as_slice())),
                            &[Token::Comma, Token::Sbc],
                        ));
                    } else {
                        break;
                    }
                }
            }
        }
    }

    Ok(ParsedValue::new(TomlValue::Array(vec), iter))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lexer::lex;

    #[test]
    pub fn simple_nested_int_arr() {
        let str = b"6,[1,2,4]]";
        let lexed = lex(str).unwrap();
        // println!("{:?}", parse_array(&lexed).unwrap().0);
    }
}
