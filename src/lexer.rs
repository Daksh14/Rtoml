use simdutf8::basic::from_utf8;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Copy, Clone, Eq)]
pub enum Token<'a> {
    Literal(&'a str),
    // equals (=)
    Eq,
    // square brackets open
    Sbo,
    // curly brackets open
    Cbo,
    // square brackets close
    Sbc,
    // curly brackets close
    Cbc,
    Hash,
    DoubleQuote,
    SingleQuote,
    LineBreak,
    CarriageReturn,
    BackSlash,
    Comma,
}

use crate::TomlError;
use Token::*;

impl<'a> Token<'a> {
    pub fn is_valid_table_name_or_key(&self) -> bool {
        if let Literal(literal) = self {
            let trimmed = literal.trim();
            let mut is_valid = false;
            trimmed.as_bytes().iter().for_each(|e| match e {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' => is_valid = true,
                _ => is_valid = false,
            });
            is_valid
        } else {
            false
        }
    }

    pub fn is_space(&self) -> bool {
        if let Literal(x) = self {
            if x.trim().is_empty() {
                return true;
            }
        }
        false
    }

    pub fn is_literal(&self) -> bool {
        matches!(self, Literal(_))
    }

    pub fn is_sbo(&self) -> bool {
        matches!(self, Sbo)
    }
}

impl<'a> Into<char> for Token<'a> {
    fn into(self) -> char {
        match self {
            Literal(_) => ' ',
            Eq => '=',
            Sbo => '[',
            Sbc => ']',
            Cbo => '{',
            Cbc => '}',
            Hash => '#',
            Comma => ',',
            DoubleQuote => '"',
            SingleQuote => '\'',
            LineBreak => '\n',
            CarriageReturn => '\r',
            BackSlash => '\\',
        }
    }
}

pub type TokenSized<'a> = (Token<'a>, usize);

pub fn lex(data: &[u8]) -> Result<Vec<TokenSized>, TomlError> {
    let mut lexemes: Vec<TokenSized> = Vec::new();
    let mut peekable = data.iter().peekable();

    let mut index = 0_usize;

    while let Some(byte) = peekable.next() {
        let entry = match get_special_byte(*byte) {
            Some(x) => {
                if let CarriageReturn = x {
                    if let Some(y) = peekable.peek().map(|x| get_special_byte(**x)).flatten() {
                        if let LineBreak = y {
                            (y, 1)
                        } else {
                            (x, 1)
                        }
                    } else {
                        (x, 1)
                    }
                } else {
                    (x, 1)
                }
            }
            _ => {
                let mut alphabetic_index = 0;
                while let Some(x) = peekable.peek() {
                    if !get_special_byte(**x).is_some() {
                        peekable.next();
                        alphabetic_index += 1;
                    } else {
                        break;
                    }
                }

                let relative_index = alphabetic_index + 1 + index;
                let string_bytes = &data[index..relative_index];
                index += alphabetic_index;

                let string = from_utf8(string_bytes)?;

                (Literal(string), string.len())
            }
        };

        index += 1;

        lexemes.push(entry);
    }

    Ok(lexemes)
}

fn get_special_byte<'a>(n: u8) -> Option<Token<'a>> {
    match n {
        b'=' => Some(Eq),
        b'[' => Some(Sbo),
        b']' => Some(Sbc),
        b'{' => Some(Cbo),
        b'}' => Some(Cbc),
        b'#' => Some(Hash),
        b',' => Some(Comma),
        b'\'' => Some(SingleQuote),
        b'"' => Some(DoubleQuote),
        b'\n' => Some(LineBreak),
        b'\r' => Some(CarriageReturn),
        b'\\' => Some(BackSlash),
        _ => None,
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal(x) => write!(f, "Literal({})", x),
            _ => {
                let character: char = (*self).into();
                write!(f, "{}", character)
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn basic_parsing() {
        let str = "hello world";
        assert_eq!((Literal(str), str.len()), lex(str.as_bytes()).unwrap()[0])
    }

    #[test]
    pub fn basic_special_chars() {
        let str = "# hello world";
        assert_eq!(
            [(Hash, 1), (Literal(" hello world"), " hello world".len())].to_vec(),
            lex(str.as_bytes()).unwrap()
        )
    }
}
