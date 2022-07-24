use crate::builtins::parse_value;
use crate::lexer::Token;
use crate::parser::r_iter::RIter;
use crate::parser::r_slice::RSlice;
use crate::{TomlError, TomlValue};

pub fn parse_array(slice: RSlice) -> Result<(TomlValue, RIter), TomlError> {
    let mut iter = RIter::from(slice);
    let mut vec = Vec::new();

    while let Some((next, _)) = iter.next() {
        match next {
            Token::Sbc => {
                break;
            }
            _ => vec.push(parse_value(iter.as_slice())?),
        }
    }

    Ok((TomlValue::Array(vec), iter))
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
