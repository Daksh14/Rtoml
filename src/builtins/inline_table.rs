use crate::assert_toml;
use crate::builtins::{ParsingError, TomlValue, Types};
use crate::lexer::Tokens;
use std::collections::HashMap;
use std::iter::Peekable;

pub struct TomlInlineTable;

impl TomlInlineTable {
    pub fn handle<S>(peekable: &mut Peekable<S>) -> Result<TomlValue, ParsingError>
    where
        S: Iterator<Item = Tokens>,
    {
        let mut found_cbc = false;
        let mut table = HashMap::new();
        while peekable.peek().is_some() {
            match peekable.peek() {
                Some(Tokens::Literal(_)) => {
                    let indetifier = peekable.next().unwrap().to_string();
                    assert_toml!(peekable.next(), Tokens::Eq);
                    table.insert(indetifier, Types::handle_value(peekable)?);
                    if let Some(Tokens::Cbc) = peekable.peek() {
                        found_cbc = true;
                        peekable.next();
                        break;
                    } else {
                        assert_toml!(peekable.next(), Tokens::Comma);
                    }
                }
                Some(Tokens::LineBreak) => {
                    if !found_cbc {
                        return Err(ParsingError::Expected(
                            Tokens::Cbc.to_string(),
                            Tokens::LineBreak.to_string(),
                        ));
                    };
                    peekable.next();
                }
                _ => {
                    return Err(ParsingError::Expected(
                        format!("Declaration or {:?}", Tokens::Cbc.to_string()),
                        peekable.next().unwrap().to_string(),
                    ));
                }
            }
        }
        if !found_cbc {
            return Err(ParsingError::Expected(
                Tokens::Cbc.to_string(),
                "None".to_string(),
            ));
        }
        Ok(TomlValue::InlineTable(table))
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::lexer::Lexer;
    use crate::TomlValue;

    #[test]
    pub fn inline_table() {
        let string = br#"{ val = "string", arr = [1,2,3] }"#;
        let lexial = Lexer::lex(string.to_vec());
        let mut peekable = lexial.into_iter().peekable();
        let parsed_table = Types::handle_value(&mut peekable);
        let mut map = HashMap::new();
        map.insert("val".to_string(), TomlValue::String("string".to_string()));

        map.insert(
            "arr".to_string(),
            TomlValue::Array(vec![
                TomlValue::Int(1),
                TomlValue::Int(2),
                TomlValue::Int(3),
            ]),
        );
        let inline_table = TomlValue::InlineTable(map);
        assert_eq!(parsed_table.unwrap(), inline_table);
    }

    #[test]
    pub fn inline_int_table() {
        let string = br#"{ val = 1 }"#;
        let lexial = Lexer::lex(string.to_vec());
        let mut peekable = lexial.into_iter().peekable();
        let parsed_table = Types::handle_value(&mut peekable);
        let mut map = HashMap::new();
        map.insert("val".to_string(), TomlValue::Int(1));
        let inline_table = TomlValue::InlineTable(map);
        assert_eq!(parsed_table.unwrap(), inline_table);
    }

    #[test]
    #[should_panic]
    pub fn inline_table_no_closing_bracket() {
        let string = br#"{ val = "string""#;
        let lexial = Lexer::lex(string.to_vec());
        let mut peekable = lexial.into_iter().peekable();
        let parsed_table = TomlInlineTable::handle(&mut peekable);
        parsed_table.ok().unwrap();
    }

    #[test]
    #[should_panic]
    pub fn inline_table_no_comma() {
        let string = br#"{ val = "string" another_val = [1,2,3] }"#;
        let lexial = Lexer::lex(string.to_vec());
        let mut peekable = lexial.into_iter().peekable();
        let parsed_table = TomlInlineTable::handle(&mut peekable);
        parsed_table.ok().unwrap();
    }
}
