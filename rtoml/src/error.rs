use crate::lexer::Token;
use crate::parser::r_iter::RIter;
use crate::parser::r_slice::RIndex;

use std::error::Error;
use std::fmt;
use std::fmt::Display;

use simdutf8::basic::Utf8Error;

#[derive(Debug)]
pub struct ErrLocation<'a> {
    index: RIndex,
    token: Token<'a>,
}

impl<'a> ErrLocation<'a> {
    pub fn new(mut iter: RIter<'a>) -> Self {
        let mut token = Token::Literal("");
        if let Some((x, _)) = iter.next() {
            token = *x;
        }
        Self {
            index: iter.index,
            token,
        }
    }
}

#[derive(Debug)]
pub enum TomlError<'a> {
    UnknownEscapeSequence(ErrLocation<'a>),
    NameUsed(ErrLocation<'a>),
    CannotParseValue(ErrLocation<'a>),
    UnspecifiedValue(ErrLocation<'a>),
    Utf8Error,
    UnexpectedCharacter(ErrLocation<'a>, &'a [Token<'a>]),
}

impl Display for TomlError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::UnknownEscapeSequence(err) => {
                write!(f, "Unknown escape sequence {} at {}", err.token, err.index)
            }
            Self::NameUsed(err) => {
                write!(
                    f,
                    "Variable name {} already used at {}",
                    err.token, err.index
                )
            }
            Self::CannotParseValue(err) => {
                write!(f, "Cannot parse value {} at {}", err.token, err.index)
            }
            Self::UnspecifiedValue(err) => {
                write!(f, "Unspecified value {} at {}", err.token, err.index)
            }
            Self::Utf8Error => {
                write!(f, "Invalid UTF8 bytes while lexing")
            }
            Self::UnexpectedCharacter(loc, expected) => {
                let mut string = String::new();
                for items in expected.iter() {
                    string.push_str(&items.to_string())
                }
                write!(
                    f,
                    "Unexpected character {}, expected {} at {}",
                    loc.token, string, loc.index
                )
            }
        }
    }
}

impl From<Utf8Error> for TomlError<'_> {
    fn from(_: Utf8Error) -> Self {
        TomlError::Utf8Error
    }
}

impl Error for TomlError<'_> {}
