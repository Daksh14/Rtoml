use crate::builtins::array::parse_array;
use crate::builtins::inline_table::parse_inline_table;
use crate::builtins::num::{parse_num_or_date, Hint};
use crate::builtins::string::parse_string;
use crate::error::ErrLocation;
use crate::lexer::Token;
use crate::parser::r_iter::RIter;
use crate::parser::r_slice::*;
use crate::{TomlError, TomlValue};

pub mod array;
pub mod inline_table;
pub mod num;
pub mod string;

pub fn parse_value(iter: RSlice) -> Result<TomlValue, TomlError> {
    let mut iter = RIter::from(iter);

    while let Some((next, _)) = iter.next() {
        match next {
            Token::DoubleQuote | Token::SingleQuote => {
                return parse_string(iter.as_slice(), *next);
            }
            Token::Sbo => return parse_array(iter.as_slice()).map(|(val, _)| val),
            Token::Cbo => return parse_inline_table(iter.as_slice()),
            Token::Literal(x) => {
                if next.is_space() {
                    continue;
                } else {
                    return check_for_other_values(x.trim(), iter.as_slice());
                }
            }
            _ => return Err(TomlError::UnspecifiedValue(ErrLocation::new(iter))),
        };
    }

    Ok(TomlValue::Int(1))
}

pub fn check_for_other_values<'a>(
    literal: &'a str,
    slice: RSlice<'a>,
) -> Result<TomlValue<'a>, TomlError<'a>> {
    let value = match literal {
        "inf" => parse_num_or_date(literal, Hint::Inf, slice)?,
        "nan" => parse_num_or_date(literal, Hint::Nan, slice)?,
        "true" => TomlValue::Boolean(true),
        "false" => TomlValue::Boolean(false),
        _ => {
            let mut iter = literal.chars();
            let prefix = iter.next();
            if let Some(first_letter) = prefix {
                match first_letter {
                    '+' => parse_num_or_date(literal, Hint::Positive, slice)?,
                    '-' => parse_num_or_date(literal, Hint::Negative, slice)?,
                    n if n.is_digit(10) => parse_num_or_date(literal, Hint::Number, slice)?,
                    _ => {
                        return Err(TomlError::CannotParseValue(ErrLocation::new(RIter::from(
                            slice,
                        ))))
                    }
                }
            } else {
                return Err(TomlError::UnspecifiedValue(ErrLocation::new(RIter::from(
                    slice,
                ))));
            }
        }
    };

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn get_tokens_from_literal(literal: &str) -> RSlice {
        let vec = vec![(Token::Literal(literal), literal.len())];
        RIter::new(&vec).as_slice()
    }
}
