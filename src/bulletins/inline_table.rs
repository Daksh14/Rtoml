use crate::bulletins::{handle_value, TomlValue};
use crate::lexer::Tokens;
use crate::{assert_toml, comp_err};
use std::collections::HashMap;
use std::iter::Peekable;

pub struct InlineTable {
    pub name: String,
    pub values: HashMap<String, TomlValue>,
}

pub fn parse_inline_table<S>(peekable: &mut Peekable<S>)
where
    S: Iterator<Item = Tokens>,
{
    let mut found_cbc = false;
    while peekable.peek().is_some() {
        if let Some(x) = peekable.next() {
            match x {
                Tokens::Literal(x) => {
                    assert_toml!(peekable.next(), Tokens::Eq);
                    handle_value(peekable);
                }
                Tokens::Cbc => found_cbc = true,
                _ => (),
            }
        }
    }
    if !found_cbc {
        comp_err!("Inline tables should be ending with `}`");
    }
}
