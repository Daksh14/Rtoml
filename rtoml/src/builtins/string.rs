use crate::error::ErrLocation;
use crate::lexer::Token;
use crate::parser::r_iter::RIter;
use crate::parser::r_slice::RSlice;
use crate::parser::ParsedValue;
use crate::{TomlError, TomlValue};

pub fn parse_string<'a>(
    slice: RSlice<'a>,
    quote_type: Token,
) -> Result<ParsedValue<'a>, TomlError<'a>> {
    let mut iter = RIter::from(slice);
    let mut string = String::new();
    let mut is_multiline = false;

    if iter.next_if_eq(quote_type) && iter.next_if_eq(quote_type) {
        is_multiline = true;

        iter.next_if_eq(Token::LineBreak);
    }

    if quote_type == Token::SingleQuote {
        parse_string_single_quotes(is_multiline, &mut string, &mut iter);
        return Ok(ParsedValue::new(TomlValue::String(string), iter));
    }

    while let Some((token, _)) = iter.next() {
        match token {
            Token::DoubleQuote => {
                if is_multiline {
                    if iter.next_if_eq(*token) && iter.next_if_eq(*token) {
                        break;
                    } else {
                        string.push((*token).into());
                        if let Some((Token::Literal(x), _)) = iter.peek() {
                            string.push_str(x);
                            iter.next();
                        } else {
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
            Token::BackSlash => {
                if let Some((token, _)) = iter.next() {
                    match token {
                        Token::Literal(literal) => {
                            let first_char = literal.as_bytes().get(0);
                            if first_char == Some(&b'u') {
                                if let Some(eight_dig) = &literal.get(1..9) {
                                    string.push(get_char_from_scalar(eight_dig, iter.as_slice())?);
                                    string.push_str(&literal[9..]);
                                } else if let Some(four_dig) = &literal.get(1..5) {
                                    string.push(get_char_from_scalar(four_dig, iter.as_slice())?);
                                    string.push_str(&literal[5..]);
                                } else {
                                    return Err(TomlError::UnknownEscapeSequence(
                                        ErrLocation::new(iter),
                                    ));
                                }
                            } else if token.is_space() {
                                trim_till_non_whitespace(&mut iter, &mut string);
                            } else {
                                string.push(escape(first_char, iter.as_slice())?);
                                string.push_str(&literal[1..]);
                            }
                        }
                        Token::DoubleQuote | Token::BackSlash => {
                            string.push((*token).into());
                        }
                        Token::LineBreak => {
                            trim_till_non_whitespace(&mut iter, &mut string);
                        }
                        _ => return Err(TomlError::UnknownEscapeSequence(ErrLocation::new(iter))),
                    }
                }
            }
            Token::LineBreak => {
                if !is_multiline {
                    return Err(TomlError::UnexpectedCharacter(
                        ErrLocation::new(iter),
                        &[Token::Literal(r#"'''"#)],
                    ));
                } else {
                    string.push((*token).into());
                }
            }
            Token::Literal(str) => {
                string.push_str(str);
            }
            _ => string.push((*token).into()),
        }
    }

    Ok(ParsedValue::new(TomlValue::String(string), iter))
}

fn parse_string_single_quotes(is_multiline: bool, string: &mut String, iter: &mut RIter) {
    if is_multiline {
        while let Some((token, _)) = iter.next() {
            match token {
                Token::SingleQuote => {
                    if iter.next_if_eq(*token) && iter.next_if_eq(*token) {
                        break;
                    } else {
                        string.push((*token).into());
                        for (token, _) in iter.by_ref() {
                            match token {
                                Token::SingleQuote => {
                                    string.push((*token).into());
                                    break;
                                }
                                Token::Literal(x) => {
                                    string.push_str(x);
                                }
                                _ => {
                                    string.push((*token).into());
                                }
                            }
                        }
                    }
                }
                Token::Literal(x) => {
                    string.push_str(x);
                }
                _ => {
                    string.push((*token).into());
                }
            }
        }
    } else {
        for (token, _) in iter.by_ref() {
            match token {
                Token::SingleQuote => break,
                Token::Literal(x) => {
                    string.push_str(x);
                }
                _ => {
                    string.push((*token).into());
                }
            }
        }
    }
}

fn escape<'a>(char: Option<&u8>, slice: RSlice<'a>) -> Result<char, TomlError<'a>> {
    let res = match char {
        Some(b'b') => '\x08',
        Some(b't') => '\x09',
        Some(b'n') => '\x0A',
        Some(b'f') => '\x0C',
        Some(b'r') => '\x0D',
        Some(_) | None => {
            return Err(TomlError::UnknownEscapeSequence(ErrLocation::new(
                RIter::from(slice),
            )))
        }
    };

    Ok(res)
}

fn get_char_from_scalar<'a>(scalar: &str, slice: RSlice<'a>) -> Result<char, TomlError<'a>> {
    u32::from_str_radix(scalar, 16)
        .map_err(|_| TomlError::UnknownEscapeSequence(ErrLocation::new(RIter::from(slice.clone()))))
        .map(|byte| {
            char::from_u32(byte).map_or_else(
                || {
                    Err(TomlError::UnknownEscapeSequence(ErrLocation::new(
                        RIter::from(slice),
                    )))
                },
                Ok,
            )
        })?
}

fn trim_till_non_whitespace<'a>(iter: &mut RIter<'a>, string: &mut String) {
    while let Some((peek, _)) = iter.peek() {
        match peek {
            peek if peek.is_space() => {
                iter.next();
            }
            Token::LineBreak => {
                iter.next();
            }
            Token::Literal(lit) => {
                string.push_str(lit.trim_start());
                iter.next();
                break;
            }
            _ => break,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;

    #[test]
    fn basic_string() {
        let lexed = &lex(br#"""hello""""#).unwrap();
        let parsed = parse_string(RIter::new(lexed).as_slice(), Token::DoubleQuote);
        assert_eq!(
            TomlValue::String(String::from(r#"tes"t"#)),
            parsed.unwrap().value
        );
    }
}
